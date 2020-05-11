
use crate::parse;
use parse::{
    Unit,
    Program,
    Instruction,
    Directive
};

pub fn write<W>(w: &mut W, program: &Program) -> Result<(), std::io::Error>
    where W: std::io::Write
{
    for unit in program {
        match unit {
            Unit::Instruction(i) => {
                use Instruction::*;
                match i {
                    Ccf => { w.write(&[0x3F])?; },
                    Cpl => { w.write(&[0x2F])?; },
                    Daa => { w.write(&[0x27])?; },
                    Di => { w.write(&[0xF3])?; },
                    Ei => { w.write(&[0xFB])?; },
                    Halt => { w.write(&[0x76])?; },
                    Nop => { w.write(&[0x00])?; },
                    Reti => { w.write(&[0xD9])?; },
                    Scf => { w.write(&[0x37])?; },
                    Stop => { w.write(&[0x10, 0x00])?; },
                    _ => ()
                }
            },

            Unit::Directive(d) => {
                use Directive::*;
                match d {
                    Byte(b) => {
                        let byte = b.unwrap_or(0x00);
                        w.write(&[byte])?;
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
            x:
                .byte 1
            y:
                .byte 1
        "#).unwrap();

        let program = parse::parse(input).unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        write(&mut bytes, &program);
        println!("{:?}", bytes);
    }
}
