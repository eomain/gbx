
use crate::token;
use token::{
    Token,
    Operation
};

pub use token::{
    Register,
    Register16
};

/// Single component of the program
#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    Label(String),
    Instruction(Instruction),
    Directive(Directive)
}

pub type Program = Vec<Unit>;

/// The Operands are either an 8-bit (or 16-bit)
/// immediate value or a register.
#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    Immediate8(u8),
    Immediate16(u16),
    Register(Register),
    Register16(Register16),
    Indirect(Register),
    Indirect16(Register16)
}

/// For instructions with variable a
/// number of operands.
#[derive(Clone, Debug, PartialEq)]
pub enum MulOperand {
    Single(Operand),
    Double(Operand, Operand)
}

/// All machine instructions.
#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Add(Operand, Operand),
    And(Operand),
    Bit(Operand, Operand),
    Ccf,
    Cp(Operand),
    Cpl,
    Daa,
    Dec(Operand),
    Di,
    Ei,
    Halt,
    Inc(Operand),
    Jp(MulOperand),
    Jr(MulOperand),
    Ld(Operand, Operand),
    Nop,
    Or(Operand),
    Pop(Operand),
    Push(Operand),
    Ret(Operand),
    Reti,
    Res(Operand, Operand),
    Rl(Operand),
    Rla(Operand),
    Rlc(Operand),
    Rlca,
    Rr(Operand),
    Rra,
    Rrc(Operand),
    Rrca,
    Rst(Operand),
    Sbc(Operand, Operand),
    Scf,
    Set(Operand, Operand),
    Sla(Operand),
    Sra(Operand),
    Srl(Operand),
    Stop,
    Sub(Operand),
    Swap(Operand),
    Xor(Operand)
}

impl From<Instruction> for Unit {
    fn from(i: Instruction) -> Self
    {
        Unit::Instruction(i)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Directive {
    Ascii(Vec<u8>),
    Asciz(Vec<u8>),
    Byte(Option<Vec<u8>>),
    Fill(usize, u8)
}

impl From<Directive> for Unit {
    fn from(d: Directive) -> Self
    {
        Unit::Directive(d)
    }
}

struct Parser {
    pos: usize,
    tokens: Vec<Token>
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self
    {
        Self {
            pos: 0,
            tokens
        }
    }

    fn look(&self) -> Option<Token>
    {
        match self.tokens.get(self.pos) {
            None => None,
            Some(token) => Some(token.clone())
        }
    }

    fn next(&mut self)
    {
        self.pos += 1;
    }

    fn ahead(&mut self) -> Option<Token>
    {
        match self.tokens.get(self.pos + 1) {
            None => None,
            Some(token) => Some(token.clone())
        }
    }
}

/// A newline following an instruction
fn newline(parser: &mut Parser) -> Result<(), ()>
{
    parser.next();
    match parser.look() {
        None | Some(Token::Newline) => (),
        _ => return Err(())
    }
    Ok(())
}

fn byte(parser: &mut Parser) -> Result<u8, ()>
{
    match parser.ahead() {
        Some(Token::Value(v)) => {
            if v > std::u8::MAX as u16 {
                return Err(())
            }
            parser.next();
            Ok(v as u8)
        },
        _ => Err(())
    }
}

fn ascii(parser: &mut Parser) -> Result<Vec<u8>, ()>
{
    match parser.ahead() {
        Some(Token::String(s)) => {
            if !s.is_ascii() {
                return Err(());
            }
            parser.next();
            Ok(s.bytes().collect())
        },
        _ => Err(())
    }
}

#[inline]
fn comma(parser: &mut Parser) -> Result<(), ()>
{
    match parser.ahead() {
        Some(Token::Comma) => {
            parser.next();
            Ok(())
        },
        _ => return Err(())
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, ()>
{
    let mut program = Vec::new();
    let mut parser = Parser::new(tokens);

    while let Some(token) = parser.look() {
        match token {
            Token::Id(s) => {
                match parser.ahead() {
                    Some(Token::Colon) => {
                        program.push(Unit::Label(s));
                        parser.next();
                    },
                    _ => return Err(())
                }
            },
            Token::Operation(o) => {
                use Operation::*;
                match o {
                    Ccf => program.push(Instruction::Ccf.into()),
                    Cpl => program.push(Instruction::Cpl.into()),
                    Daa => program.push(Instruction::Daa.into()),
                    Di => program.push(Instruction::Di.into()),
                    Ei => program.push(Instruction::Ei.into()),
                    Halt => program.push(Instruction::Halt.into()),
                    Nop => program.push(Instruction::Nop.into()),
                    Reti => program.push(Instruction::Reti.into()),
                    Rlca => program.push(Instruction::Rlca.into()),
                    Rra => program.push(Instruction::Rra.into()),
                    Rrca => program.push(Instruction::Rrca.into()),
                    Scf => program.push(Instruction::Scf.into()),
                    Stop => program.push(Instruction::Stop.into()),
                    _ => ()
                }
                newline(&mut parser)?;
            },
            Token::Directive(d) => {
                use token::Directive as Direc;
                match d {
                    Direc::Ascii => {
                        let bytes = ascii(&mut parser)?;
                        program.push(Directive::Ascii(bytes).into());
                    },
                    Direc::Asciz => {
                        let bytes = ascii(&mut parser)?;
                        program.push(Directive::Asciz(bytes).into());
                    },
                    Direc::Byte => {
                        match byte(&mut parser) {
                            Err(_) => program.push(Directive::Byte(None).into()),
                            Ok(b) => {
                                let mut bytes = vec![b];
                                while let Ok(_) = comma(&mut parser) {
                                    let byte = byte(&mut parser)?;
                                    bytes.push(byte);
                                }
                                program.push(Directive::Byte(Some(bytes)).into());
                            }
                        }
                    },
                    Direc::Fill => {
                        let size = match parser.ahead() {
                            Some(Token::Value(v)) => {
                                parser.next();
                                v as usize
                            },
                            _ => return Err(())
                        };

                        comma(&mut parser)?;

                        let byte = byte(&mut parser)?;

                        program.push(Directive::Fill(size, byte).into());
                    },
                    _ => ()
                }
                newline(&mut parser)?;
            },
            Token::Newline => (),
            _ => return Err(())
        }
        parser.next();
    }

    Ok(program)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token;

    #[test]
    fn program()
    {
        let input = token::scan(r#"
            _start: ; program entry point
                ; no operation
                nop
                ; disable interrupts
                di
            fini:
                ; halt the cpu
                halt
        "#).unwrap();

        let program = parse(input).unwrap();
        println!("{:?}", program);
    }
}
