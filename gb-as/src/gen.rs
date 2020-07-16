
use crate::parse;
use parse::{
    Unit,
    Program,
    Operand,
    Register,
    Register16,
    FlagRegister as Flag,
    Instruction,
    Directive
};

use byteorder::{
    LittleEndian,
    WriteBytesExt
};

fn and_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
        Operand::Immediate8(u) => { w.write(&[0xE6, *u])?; },
        Operand::Register(r) => {
            use Register::*;
            match r {
                A => { w.write(&[0xA7])?; },
                B => { w.write(&[0xA0])?; },
                C => { w.write(&[0xA1])?; },
                D => { w.write(&[0xA2])?; },
                E => { w.write(&[0xA3])?; },
                H => { w.write(&[0xA4])?; },
                L => { w.write(&[0xA5])?; }
            }
        },
        Operand::Indirect16(Register16::HL) => { w.write(&[0xA6])?; },
        _ => unreachable!()
    }
    Ok(())
}

fn call_1_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
        Operand::Immediate16(u) => {
            w.write(&[0xCD])?;
            w.write_u16::<LittleEndian>(*u)?;
        },
        _ => unreachable!()
    }
    Ok(())
}

fn call_2_write<W>(w: &mut W, op1: &Operand, op2: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match (op1, op2) {
        (Operand::Flag(f), Operand::Immediate16(u)) => {
            match f {
                Flag::Z  => { w.write(&[0xCC])?; },
                Flag::NZ => { w.write(&[0xC4])?; },
                Flag::CR  => { w.write(&[0xDC])?; },
                Flag::NC => { w.write(&[0xD4])?; },
                _  => unreachable!()
            }
            w.write_u16::<LittleEndian>(*u)?;
        },
        _ => unreachable!()
    }
    Ok(())
}


fn cp_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
        Operand::Immediate8(u) => { w.write(&[0xFE, *u])?; },
        Operand::Register(r) => {
            use Register::*;
            match r {
                A => { w.write(&[0xBF])?; },
                B => { w.write(&[0xB8])?; },
                C => { w.write(&[0xB9])?; },
                D => { w.write(&[0xBA])?; },
                E => { w.write(&[0xBB])?; },
                H => { w.write(&[0xBC])?; },
                L => { w.write(&[0xBD])?; }
            }
        },
        Operand::Indirect16(Register16::HL) => { w.write(&[0xBE])?; },
        _ => unreachable!()
    }
    Ok(())
}

fn dec_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
        Operand::Register(r) => {
            use Register::*;
            match r {
                A => { w.write(&[0x3D])?; },
                B => { w.write(&[0x05])?; },
                C => { w.write(&[0x0D])?; },
                D => { w.write(&[0x15])?; },
                E => { w.write(&[0x1D])?; },
                H => { w.write(&[0x25])?; },
                L => { w.write(&[0x2D])?; }
            }
        },
        Operand::Register16(r) => {
            use Register16::*;
            match r {
                BC => { w.write(&[0x0B])?; },
                DE => { w.write(&[0x1B])?; },
                HL => { w.write(&[0x2B])?; },
                SP => { w.write(&[0x3B])?; },
                _ => unreachable!()
            }
        },
        Operand::Indirect16(Register16::HL) => { w.write(&[0x35])?; },
        _ => unreachable!()
    }
    Ok(())
}

fn inc_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
        Operand::Register(r) => {
            use Register::*;
            match r {
                A => { w.write(&[0x3C])?; },
                B => { w.write(&[0x04])?; },
                C => { w.write(&[0x0C])?; },
                D => { w.write(&[0x14])?; },
                E => { w.write(&[0x1C])?; },
                H => { w.write(&[0x24])?; },
                L => { w.write(&[0x2C])?; }
            }
        },
        Operand::Register16(r) => {
            use Register16::*;
            match r {
                BC => { w.write(&[0x03])?; },
                DE => { w.write(&[0x13])?; },
                HL => { w.write(&[0x23])?; },
                SP => { w.write(&[0x33])?; },
                _ => unreachable!()
            }
        },
        Operand::Indirect16(Register16::HL) => { w.write(&[0x34])?; },
        _ => unreachable!()
    }
    Ok(())
}

fn jp_1_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
        Operand::Immediate16(u) => {
            w.write(&[0xC3])?;
            w.write_u16::<LittleEndian>(*u)?;
        },
        Operand::Indirect16(Register16::HL) => { w.write(&[0xE9])?; },
        _ => unreachable!()
    }
    Ok(())
}

fn jp_2_write<W>(w: &mut W, op1: &Operand, op2: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match (op1, op2) {
        (Operand::Flag(f), Operand::Immediate16(u)) => {
            match f {
                Flag::Z  => { w.write(&[0xCA])?; },
                Flag::NZ => { w.write(&[0xC2])?; },
                Flag::CR  => { w.write(&[0xDA])?; },
                Flag::NC => { w.write(&[0xD2])?; },
                _  => unreachable!()
            }
            w.write_u16::<LittleEndian>(*u)?;
        },
        _ => unreachable!()
    }
    Ok(())
}

fn jr_1_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
        Operand::Immediate8(u) => { w.write(&[0x18, *u])?; },
        _ => unreachable!()
    }
    Ok(())
}

fn jr_2_write<W>(w: &mut W, op1: &Operand, op2: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match (op1, op2) {
        (Operand::Flag(f), Operand::Immediate8(u)) => {
            match f {
                Flag::Z  => { w.write(&[0x28])?; },
                Flag::NZ => { w.write(&[0x20])?; },
                Flag::CR  => { w.write(&[0x38])?; },
                Flag::NC => { w.write(&[0x30])?; },
                _  => unreachable!()
            }
            w.write(&[*u])?;
        },
        _ => unreachable!()
    }
    Ok(())
}

fn or_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
        Operand::Immediate8(u) => { w.write(&[0xF6, *u])?; },
        Operand::Register(r) => {
            use Register::*;
            match r {
                A => { w.write(&[0xB7])?; },
                B => { w.write(&[0xB0])?; },
                C => { w.write(&[0xB1])?; },
                D => { w.write(&[0xB2])?; },
                E => { w.write(&[0xB3])?; },
                H => { w.write(&[0xB4])?; },
                L => { w.write(&[0xB5])?; }
            }
        },
        Operand::Indirect16(Register16::HL) => { w.write(&[0xB6])?; },
        _ => unreachable!()
    }
    Ok(())
}

fn pop_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
        Operand::Register16(Register16::AF) => { w.write(&[0xF1])?; },
        Operand::Register16(Register16::BC) => { w.write(&[0xC1])?; },
        Operand::Register16(Register16::DE) => { w.write(&[0xD1])?; },
        Operand::Register16(Register16::HL) => { w.write(&[0xE1])?; },
        _ => unreachable!()
    }
    Ok(())
}

fn push_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
        Operand::Register16(Register16::AF) => { w.write(&[0xF5])?; },
        Operand::Register16(Register16::BC) => { w.write(&[0xC5])?; },
        Operand::Register16(Register16::DE) => { w.write(&[0xD5])?; },
        Operand::Register16(Register16::HL) => { w.write(&[0xE5])?; },
        _ => unreachable!()
    }
    Ok(())
}

fn ret_1_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
        Operand::Flag(Flag::Z)  => { w.write(&[0xC8])?; },
        Operand::Flag(Flag::NZ) => { w.write(&[0xC0])?; },
        Operand::Flag(Flag::CR)  => { w.write(&[0xD8])?; },
        Operand::Flag(Flag::NC) => { w.write(&[0xD0])?; },
        _ => unreachable!()
    }
    Ok(())
}

fn xor_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
        Operand::Immediate8(u) => { w.write(&[0xEE, *u])?; },
        Operand::Register(r) => {
            use Register::*;
            match r {
                A => { w.write(&[0xAF])?; },
                B => { w.write(&[0xA8])?; },
                C => { w.write(&[0xA9])?; },
                D => { w.write(&[0xAA])?; },
                E => { w.write(&[0xAB])?; },
                H => { w.write(&[0xAC])?; },
                L => { w.write(&[0xAD])?; }
            }
        },
        Operand::Indirect16(Register16::HL) => { w.write(&[0xAE])?; },
        _ => unreachable!()
    }
    Ok(())
}

pub fn write<W>(w: &mut W, program: &Program) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    for unit in program {
        match unit {
            Unit::Instruction(i) => {
                use Instruction::*;
                match i {
                    And(o) => and_write(w, o)?,
                    Call_1(o) => call_1_write(w, o)?,
                    Call_2(o1, o2) => call_2_write(w, o1, o2)?,
                    Ccf => { w.write(&[0x3F])?; },
                    Cpl => { w.write(&[0x2F])?; },
                    Daa => { w.write(&[0x27])?; },
                    Dec(o) => dec_write(w, o)?,
                    Di => { w.write(&[0xF3])?; },
                    Ei => { w.write(&[0xFB])?; },
                    Halt => { w.write(&[0x76])?; },
                    Inc(o) => inc_write(w, o)?,
                    Jp_1(o) => jp_1_write(w, o)?,
                    Jp_2(o1, o2) => jp_2_write(w, o1, o2)?,
                    Jr_1(o) => jr_1_write(w, o)?,
                    Jr_2(o1, o2) => jr_2_write(w, o1, o2)?,
                    Nop => { w.write(&[0x00])?; },
                    Or(o) => or_write(w, o)?,
                    Pop(o) => pop_write(w, o)?,
                    Push(o) => push_write(w, o)?,
                    Ret => { w.write(&[0xC9])?; },
                    Ret_1(o) => ret_1_write(w, o)?,
                    Reti => { w.write(&[0xD9])?; },
                    Scf => { w.write(&[0x37])?; },
                    Stop => { w.write(&[0x10, 0x00])?; },
                    Xor(o) => xor_write(w, o)?,
                    _ => ()
                }
            },

            Unit::Directive(d) => {
                use Directive::*;
                match d {
                    Ascii(string) | Utf8(string) => {
                        w.write(&string)?;
                    },
                    Asciz(string) => {
                        w.write(&string)?;
                        w.write(&[0x00]);
                    },
                    Byte(bytes) => {
                        match bytes {
                            None => { w.write(&[0x00])?; },
                            Some(bytes) => { w.write(bytes)?; }
                        }
                    },
                    Fill(size, byte) => {
                        const MAX: usize = 0xFF;
                        if *size <= MAX {
                            let bytes = vec![*byte; *size];
                            w.write(bytes.as_slice())?;
                        } else {
                            let bytes = vec![*byte; MAX];
                            let times = size / MAX;
                            let rem = size % MAX;
                            for n in 1..=times {
                                w.write(&bytes)?;
                            }
                            let bytes = vec![*byte; rem];
                            w.write(&bytes)?;
                        }
                    }
                }
            },

            _ => ()
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token;

    #[test]
    fn codegen()
    {
        let input = token::scan(r#"
            .text
            _start: ; program entry point
                ; disable interrupts
                di
                ; no operation
                nop
            fini:
                ; halt the cpu
                halt
            .data
            xyz:
                .byte 0x00, 0x00, 0x00
            x:
                .byte 1
            y:
                .byte 1
            msg:
                .asciz "hello world"
                .utf8 "世界"
        "#).unwrap();

        let program = parse::parse(input).unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        write(&mut bytes, &program);
        println!("{:?}", bytes);
    }

    #[test]
    fn opcode()
    {
        let input = token::scan(r#"
            .text
            _start:
                push af
                and b
                or a
                halt
                nop
        "#).unwrap();

        let program = parse::parse(input).unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        write(&mut bytes, &program);
        println!("{:?}", bytes);
    }
}
