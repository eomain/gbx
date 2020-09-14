
use std::{
    path::PathBuf,
    collections::{
        HashMap,
        HashSet
    }
};
use crate::token;
use token::{
    Token,
    Operation
};

pub use token::{
    Register,
    Register16,
    FlagRegister
};

trait Bytes {
    fn bytes(&self) -> u16;
}

/// Single component of the program
#[derive(Clone, Debug, PartialEq)]
pub enum Unit {
    Instruction(Instruction),
    Directive(Directive)
}

impl Unit {
    fn bytes(&self, loc: u16) -> u16
    {
        use Unit::*;
        match self {
            Instruction(i) => i.bytes(),
            Directive(d) => d.bytes(loc),
            _ => 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub units: Vec<Unit>,
    location: u16
}

impl Program {
    fn new() -> Self
    {
        Self {
            units: Vec::new(),
            location: 0
        }
    }

    fn push(&mut self, unit: Unit)
    {
        self.location += unit.bytes(self.location);
        self.units.push(unit);
    }
}

pub type Table = HashMap<String, u16>;

/// The Operands are either an 8-bit (or 16-bit)
/// immediate value or a register.
#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    Immediate8(u8),
    Immediate16(u16),
    Register(Register),
    Register16(Register16),
    Indirect(Register),
    Indirect16(Register16),
    Flag(FlagRegister),
    Symbol(String),
}

impl Bytes for Operand {
    fn bytes(&self) -> u16
    {
        use Operand::*;
        match self {
            Immediate8(_) => 1,
            Immediate16(_) | Symbol(_) => 2,
            _ => 0,
        }
    }
}

/// All machine instructions.
#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    Add(Operand, Operand),
    And(Operand),
    Bit(Operand, Operand),
    Call_1(Operand),
    Call_2(Operand, Operand),
    Ccf,
    Cp(Operand),
    Cpl,
    Daa,
    Dec(Operand),
    Di,
    Ei,
    Halt,
    Inc(Operand),
    Jp_1(Operand),
    Jp_2(Operand, Operand),
    Jr_1(Operand),
    Jr_2(Operand, Operand),
    Ld(Operand, Operand),
    Nop,
    Or(Operand),
    Pop(Operand),
    Push(Operand),
    Ret,
    Ret_1(Operand),
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

impl Bytes for Instruction {
    fn bytes(&self) -> u16
    {
        use Instruction::*;
        match self {
            Add(a, b) | Bit(a, b) | Call_2(a, b) |
            Jp_2(a, b) | Jr_2(a, b) | Ld(a, b) |
            Res(a, b) | Sbc(a, b) | Set(a, b) => 1 + a.bytes() + b.bytes(),
            And(a) | Call_1(a) | Cp(a) |
            Dec(a) | Inc(a) | Jp_1(a) | Jr_1(a) |
            Or(a) | Pop(a) | Push(a) |
            Ret_1(a) | Rl(a) | Rla(a) |
            Rr(a) | Rrc(a) | Rst(a) |
            Sla(a) | Sra(a) | Srl(a) |
            Sub(a) | Swap(a) | Xor(a) => 1 + a.bytes(),
            Stop => 2,
            _ => 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Directive {
    Ascii(Vec<u8>),
    Asciz(Vec<u8>),
    Byte(Option<Vec<u8>>),
    Fill(usize, u8),
    Org(usize, u8),
    Utf8(Vec<u8>)
}

impl From<Directive> for Unit {
    fn from(d: Directive) -> Self
    {
        Unit::Directive(d)
    }
}

impl Directive {
    fn bytes(&self, loc: u16) -> u16
    {
        use Directive::*;
        match self {
            Ascii(v) | Asciz(v) | Utf8(v) => v.len() as u16,
            Byte(o) => match o {
                None => 0,
                Some(v) => v.len() as u16
            },
            Fill(size, _) | Org(size, _) => *size as u16
        }
    }
}

struct Parser {
    pos: usize,
    tokens: Vec<Token>,
    symbols: Table
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self
    {
        Self {
            pos: 0,
            tokens,
            symbols: HashMap::new()
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

    fn include(&mut self, mut tokens: Vec<Token>, rewind: usize)
    {
        let mut end = self.tokens.split_off(self.pos + 1);
        self.tokens.split_off(self.pos - 1);
        self.tokens.append(&mut tokens);
        self.tokens.append(&mut end);
        self.pos -= rewind;
    }
}

fn id(parser: &mut Parser) -> Result<String, ()>
{
    parser.next();
    match parser.look() {
        Some(Token::Id(s)) => Ok(s.clone()),
        _ => return Err(())
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

fn word(parser: &mut Parser) -> Result<Operand, ()>
{
    Ok(match parser.ahead() {
        Some(Token::Value(v)) => {
            parser.next();
            Operand::Immediate16(v)
        },
        Some(Token::Id(s)) => {
            parser.next();
            Operand::Symbol(s.into())
        },
        _ => return Err(())
    })
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

fn utf8(parser: &mut Parser) -> Result<Vec<u8>, ()>
{
    match parser.ahead() {
        Some(Token::String(s)) => {
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

fn reg16_not_sp_pc(parser: &mut Parser) -> Result<Operand, ()>
{
    match parser.ahead() {
        Some(Token::Register16(Register16::SP)) |
        Some(Token::Register16(Register16::PC)) => {
            Err(())
        },
        Some(Token::Register16(r)) => {
            parser.next();
            Ok(Operand::Register16(r))
        },
        _ => Err(())
    }
}

fn reg_any_reg16_hl(parser: &mut Parser) -> Result<Operand, ()>
{
    match parser.ahead() {
        Some(Token::Register(r)) => {
            parser.next();
            Ok(Operand::Register(r))
        },
        Some(Token::Register16(Register16::HL)) => {
            parser.next();
            Ok(Operand::Indirect16(Register16::HL))
        },
        _ => return Err(())
    }
}

fn value_byte(parser: &mut Parser) -> Result<(usize, u8), ()>
{
    let size = match parser.ahead() {
        Some(Token::Value(v)) => {
            parser.next();
            v as usize
        },
        _ => return Err(())
    };

    comma(parser)?;

    let byte = byte(parser)?;
    Ok((size, byte))
}

fn call(parser: &mut Parser) -> Result<Instruction, ()>
{
    let operand = word(parser)?;
    Ok(Instruction::Call_1(operand))
}

fn jp(parser: &mut Parser) -> Result<Instruction, ()>
{
    let operand = word(parser)?;
    Ok(Instruction::Jp_1(operand))
}

fn ret(parser: &mut Parser) -> Result<Instruction, ()>
{
    match parser.ahead() {
        Some(Token::Newline) => Ok(Instruction::Ret),
        Some(Token::Flag(f)) => {
            use FlagRegister::*;
            match f {
                Z | NZ | CR | NC => {
                    parser.next();
                    Ok(Instruction::Ret_1(Operand::Flag(f)))
                },
                _ => Err(())
            }
        },
        _ => Err(())
    }
}

fn ref_operand(parser: &mut Parser, name: &str) -> Operand
{
    match parser.symbols.get(name) {
        None => {
            // TODO
            unreachable!();
        },
        Some(u) => {
            Operand::Immediate16(*u)
        }
    }
}

fn ref_labels(parser: &mut Parser, program: &mut Program)
{
    for u in program.units.iter_mut() {
        match u {
            Unit::Instruction(i) => {
                use Instruction::*;
                match i {
                    Call_1(Operand::Symbol(s)) => { *i = Call_1(ref_operand(parser, s)); },
                    Jp_1(Operand::Symbol(s)) => { *i = Jp_1(ref_operand(parser, s)); },
                    _ => ()
                }
            },
            _ => ()
        }
    }
}

pub fn parse(mut includes: HashSet<PathBuf>, tokens: Vec<Token>) -> Result<(Program, Table), ()>
{
    let mut program = Program::new();
    let mut parser = Parser::new(tokens);

    while let Some(token) = parser.look() {
        match token {
            Token::Id(s) => {
                match parser.ahead() {
                    Some(Token::Colon) => {
                        if parser.symbols.contains_key(&s) {
                            // TODO
                            return Err(());
                        }
                        parser.symbols.insert(s.to_string(), program.location);
                        parser.next();
                        parser.next();
                        continue;
                    },
                    _ => return Err(())
                }
            },
            Token::Operation(o) => {
                use Operation::*;
                match o {
                    And  => program.push(Instruction::And(reg_any_reg16_hl(&mut parser)?).into()),
                    Call => program.push(call(&mut parser)?.into()),
                    Ccf  => program.push(Instruction::Ccf.into()),
                    Cpl  => program.push(Instruction::Cpl.into()),
                    Daa  => program.push(Instruction::Daa.into()),
                    Dec  => program.push(Instruction::Dec(reg_any_reg16_hl(&mut parser)?).into()),
                    Di   => program.push(Instruction::Di.into()),
                    Ei   => program.push(Instruction::Ei.into()),
                    Halt => program.push(Instruction::Halt.into()),
                    Inc  => program.push(Instruction::Inc(reg_any_reg16_hl(&mut parser)?).into()),
                    Jp   => program.push(jp(&mut parser)?.into()),
                    Nop  => program.push(Instruction::Nop.into()),
                    Or   => program.push(Instruction::Or(reg_any_reg16_hl(&mut parser)?).into()),
                    Pop  => program.push(Instruction::Pop(reg16_not_sp_pc(&mut parser)?).into()),
                    Push => program.push(Instruction::Push(reg16_not_sp_pc(&mut parser)?).into()),
                    Ret  => program.push(ret(&mut parser)?.into()),
                    Reti => program.push(Instruction::Reti.into()),
                    Rlca => program.push(Instruction::Rlca.into()),
                    Rra  => program.push(Instruction::Rra.into()),
                    Rrca => program.push(Instruction::Rrca.into()),
                    Scf  => program.push(Instruction::Scf.into()),
                    Stop => program.push(Instruction::Stop.into()),
                    Xor  => program.push(Instruction::Xor(reg_any_reg16_hl(&mut parser)?).into()),
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
                        let (size, byte) = value_byte(&mut parser)?;
                        program.push(Directive::Fill(size, byte).into());
                    },
                    Direc::Org => {
                        let (pos, byte) = value_byte(&mut parser)?;
                        if pos >= program.location as usize {
                            program.push(Directive::Org((pos - program.location as usize), byte).into());
                        } else {
                            return Err(());
                        }
                    },
                    Direc::Set => {
                        let symbol = id(&mut parser)?;
                        comma(&mut parser)?;
                        match parser.ahead() {
                            Some(Token::Value(v)) => {
                                parser.symbols.insert(symbol, v);
                                parser.next();
                            },
                            _ => return Err(())
                        }
                    },
                    Direc::Use => {
                        let name = match String::from_utf8(utf8(&mut parser)?) {
                            Err(_) => return Err(()),
                            Ok(s) => s
                        };

                        let mut path: PathBuf = match crate::path(&name) {
                            Err(_) => return Err(()),
                            Ok(path) => path
                        };

                        if includes.contains(&path) {
                            return Err(());
                        }

                        let read = crate::read_file_token(&name);
                        includes.insert(path);
                        if let Ok(tokens) = read {
                            parser.include(tokens, 1);
                            continue;
                        } else {
                            return Err(());
                        }
                    },
                    Direc::Utf8 => {
                        let bytes = utf8(&mut parser)?;
                        program.push(Directive::Utf8(bytes).into());
                    },
                    _ => ()
                }
                newline(&mut parser)?;
            },
            Token::Newline => {
                parser.next();
                continue;
            },
            _ => return Err(())
        }

        parser.next();
    }

    ref_labels(&mut parser, &mut program);
    Ok((program, parser.symbols))
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
