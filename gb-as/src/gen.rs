
use crate::parse;
use parse::{
    Unit,
    Program,
    Operand,
    Register,
    Register16,
    Instruction,
    Directive
};

fn and_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
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
        //Operand::Immediate8(u) => { w.write(&[0xE6, *u])?; },
        Operand::Indirect16(Register16::HL) => { w.write(&[0xA6])?; },
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
        Operand::Indirect16(Register16::HL) => { w.write(&[0x34])?; },
        _ => unreachable!()
    }
    Ok(())
}

fn or_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
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
        //Operand::Immediate8(u) => { w.write(&[0xF6, *u])?; },
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

fn xor_write<W>(w: &mut W, operand: &Operand) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    match operand {
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
        //Operand::Immediate8(u) => { w.write(&[0xEE, *u])?; },
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
                    Ccf => { w.write(&[0x3F])?; },
                    Cpl => { w.write(&[0x2F])?; },
                    Daa => { w.write(&[0x27])?; },
                    Dec(o) => dec_write(w, o)?,
                    Di => { w.write(&[0xF3])?; },
                    Ei => { w.write(&[0xFB])?; },
                    Halt => { w.write(&[0x76])?; },
                    Inc(o) => inc_write(w, o)?,
                    Nop => { w.write(&[0x00])?; },
                    Or(o) => or_write(w, o)?,
                    Pop(o) => pop_write(w, o)?,
                    Push(o) => push_write(w, o)?,
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
