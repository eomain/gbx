
use std::collections::BTreeMap;
use serde::{
    Serialize,
    Deserialize
};

const MAGIC: &'static str = "GB-O!";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// A library containing a unit of code
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Library<'a> {
    /// Magic number
    magic: &'a str,
    /// The library version
    version: &'a str,
    /// Section information area
    section: Section
}

impl<'a> Library<'a> {
    /// Create a new library
    pub fn new(section: Section) -> Self
    {
        Self {
            magic: MAGIC,
            version: VERSION,
            section
        }
    }

    /// Read a library from a sequence of bytes
    pub fn read(bin: &'a [u8]) -> Result<Self, ()>
    {
        match bincode::deserialize(bin) {
            Err(_) => Err(()),
            Ok(lib) => Ok(lib)
        }
    }

    /// Write out a library into a sequence of bytes
    pub fn write(&self) -> Result<Vec<u8>, ()>
    {
        match bincode::serialize(self) {
            Err(_) => Err(()),
            Ok(bin) => Ok(bin)
        }
    }
}

/// The various sections of the library
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
    /// Contains executable code
    text: Text
}

impl Section {
    pub fn new(text: Text) -> Self
    {
        Self {
            text
        }
    }
}

/// Stores the relative address of a section symbol
pub type Addr = u16;

/// Maps names (symbols) to address's
pub type Sym = String;

/// The text (code) section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Text {
    bin: Vec<u8>,
    sym: BTreeMap<Sym, Addr>
}

impl Text {
    pub fn new(bin: Vec<u8>) -> Self
    {
        Self {
            bin,
            sym: BTreeMap::new()
        }
    }

    pub fn sym<T>(&mut self, sym: T, addr: Addr)
        where T: Into<Sym>
    {
        self.sym.insert(sym.into(), addr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lib_test()
    {
        let mut text = Text::new(vec![0x10, 0x10, 0x10]);
        text.sym("_start", 0x01);
        let sect = Section::new(text);
        let lib = Library::new(sect);
        let bin = lib.write().unwrap();
        let lib = Library::read(&bin).unwrap();
        assert_eq!(lib.section.text.bin, [0x10, 0x10, 0x10]);
    }
}
