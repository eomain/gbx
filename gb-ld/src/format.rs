
extern crate byteorder;

use std::io;
use std::io::Seek;
use std::io::Write;
use std::fmt;
use std::fmt::Display;
use byteorder::{LittleEndian, WriteBytesExt};

const CART_INFO_ADDR: u16 = 0x100;

// opcode for nop instruction
const NOP: u8 = 0x00;
// opcode for jp instruction
const JP:  u8 = 0xC3;

/// the system start-up logo
static LOGO_GRAPHIC: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
];

/// set the entry point to the given address
macro_rules! entry {
    ($x: expr) => {
        {
            let mut v = Vec::new();
            v.write_u16::<LittleEndian>($x).unwrap();
            let entry:[u8; 4] = [ NOP, JP, v[0], v[1] ];
            entry
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// which variant of the console
enum ConsoleVariant {
    /// the Game Boy color
    Color = 0x80,
    /// not a Game Boy color
    Other = 0x00
}

#[derive(Clone, Copy)]
/// meta data contained within each cart
struct CartInfo {
    /// beginning of code execution point
    entry: [u8; 4],
    /// contains the graphic logo
    logo: &'static [u8; 48],
    // console variant
    pub variant: ConsoleVariant,
    // type of cartridge
    pub cart: CartType
}

impl CartInfo {
    fn new(cart: CartType, variant: ConsoleVariant, start: u16) -> Self
    {
        Self {
            entry: entry!(start),
            logo: &LOGO_GRAPHIC,
            variant,
            cart
        }
    }

    fn write<W: Write>(&self, w: &mut W) -> io::Result<()>
    {
        w.write(&self.entry)?;
        w.write(self.logo)?;
        Ok(())
    }
}

impl From<CartInfo> for Vec<u8> {
    fn from(info: CartInfo) -> Self
    {
        let mut v = Vec::new();
        for b in &info.entry {
            v.push(*b);
        }
        for b in info.logo.iter() {
            v.push(*b);
        }
        v
    }
}

impl Display for CartInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{:?}", self.entry)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// the type of Game Boy cartridge
enum CartType {
    /// Rom-only cartridge
    ROM,
    /// Memory bank controller 1
    MBC1,
    /// Memory bank controller 2
    MBC2,
}

pub struct Cart {
    info: CartInfo,
    data: Vec<u8>
}

impl Cart {
    fn new(info: CartInfo) -> Self
    {
        Self {
            info,
            data: Vec::new()
        }
    }

    fn write<W: Write + Seek>(&self, w: &mut W) -> io::Result<()>
    {
        w.seek(io::SeekFrom::Start((CART_INFO_ADDR - 1) as u64));
        self.info.write(w)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::fs::File;
    use crate::format::*;

    #[test]
    fn cart_info()
    {
        let ctype = CartType::ROM;
        let variant = ConsoleVariant::Other;
        let start = 0x100;
        let info = CartInfo::new(ctype, variant, start);

        let cart = Cart::new(info);
        let mut f = File::create("out.gb").unwrap();
        cart.write(&mut f);
    }
}
