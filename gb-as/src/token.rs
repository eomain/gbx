
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// A code label
    Id(String),
    /// A value
    Value(u16),
    /// A string literal
    String(String),
    /// An 8-bit register
    Register(Register),
    /// A 16-bit register
    Register16(Register16),
    /// A flag register
    Flag(FlagRegister),
    /// Name of machine instruction
    Operation(Operation),
    /// Assembler directive
    Directive(Directive),
    /// Current location
    Location,
    /// A comma separator
    Comma,
    /// A colon
    Colon,
    /// A newline character
    Newline
}

/// The Game Boy has eight 8-bit registers.
#[derive(Clone, Debug, PartialEq)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L
}

impl From<Register> for Token {
    fn from(r: Register) -> Self
    {
        Token::Register(r)
    }
}

/// There are two 16-bit registers, the SP and
/// the PC. However the 8-bits registers can also
/// be paired as 16-bit ones.
#[derive(Clone, Debug, PartialEq)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}

impl From<Register16> for Token {
    fn from(r: Register16) -> Self
    {
        Token::Register16(r)
    }
}

/// The flag register.
#[derive(Clone, Debug, PartialEq)]
pub enum FlagRegister {
    /// Zero flag
    Z,
    /// Not zero
    NZ,
    /// Subtract flag
    N,
    /// Half-carry flag
    HC,
    /// Carry flag
    CR,
    /// Not carry
    NC
}

impl From<FlagRegister> for Token {
    fn from(f: FlagRegister) -> Self
    {
        Token::Flag(f)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Add,
    And,
    Bit,
    Call,
    Ccf,
    Cp,
    Cpl,
    Daa,
    Dec,
    Di,
    Ei,
    Halt,
    Inc,
    Jp,
    Jr,
    Ld,
    Nop,
    Or,
    Pop,
    Push,
    Ret,
    Reti,
    Res,
    Rl,
    Rla,
    Rlc,
    Rlca,
    Rr,
    Rra,
    Rrc,
    Rrca,
    Rst,
    Sbc,
    Scf,
    Set,
    Sla,
    Sra,
    Srl,
    Stop,
    Sub,
    Swap,
    Xor
}

impl From<Operation> for Token {
    fn from(o: Operation) -> Self
    {
        Token::Operation(o)
    }
}

/// The assembler directives
#[derive(Clone, Debug, PartialEq)]
pub enum Directive {
    Ascii,
    Asciz,
    Byte,
    Data,
    Fill,
    Org,
    Text,
    Use,
    Utf8
}

impl From<Directive> for Token {
    fn from(d: Directive) -> Self
    {
        Token::Directive(d)
    }
}

/// Tokenize the input stream
struct Tokenizer {
    line: usize,
    offset: usize,
    index: usize,
    input: Vec<char>,
    string: String
}

impl Tokenizer {
    fn new(input: Vec<char>) -> Self
    {
        Self {
            line: 1,
            offset: 1,
            index: 0,
            input,
            string: String::new()
        }
    }

    fn read(&self) -> Option<char>
    {
        match self.input.get(self.index) {
            None => None,
            Some(c) => Some(*c)
        }
    }

    fn next(&mut self) -> Option<char>
    {
        self.index += 1;
        match self.read() {
            None => None,
            Some(c) => {
                if c == '\n' {
                    self.line += 1;
                    self.offset = 1;
                } else {
                    self.offset += 1;
                }
                Some(c)
            }
        }
    }

    fn ahead(&mut self) -> Option<char>
    {
        match self.input.get(self.index + 1) {
            None => None,
            Some(c) => Some(*c)
        }
    }

    fn read_while<F>(&mut self, f: F)
        where F: Fn(char) -> bool
    {
        let c = self.read().unwrap();
        self.string.push(c);
        while let Some(c) = self.ahead() {
            if !f(c) {
                break;
            }
            self.string.push(c);
            self.next();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Error {
    Directive
}

#[inline]
fn alpha(c: char) -> bool
{
    c.is_ascii_alphabetic()
}

#[inline]
fn numeric(c: char) -> bool
{
    c.is_ascii_digit()
}

#[inline]
fn base_2(c: char) -> bool
{
    (c == '0' || c == '1')
}

fn binary(tokenizer: &mut Tokenizer) -> Result<Token, ()>
{
    match tokenizer.ahead() {
        None => return Err(()),
        Some(c) => if !base_2(c) {
            return Err(());
        }
    }

    tokenizer.next();
    tokenizer.string.clear();
    tokenizer.read_while(base_2);
    let num = std::mem::take(&mut tokenizer.string);

    Ok(Token::Value(u16::from_str_radix(&num, 2).unwrap()))
}

fn octal(tokenizer: &mut Tokenizer) -> Token
{
    tokenizer.string.clear();
    tokenizer.read_while(numeric);
    let num = std::mem::take(&mut tokenizer.string);
    Token::Value(u16::from_str_radix(&num, 8).unwrap())
}

fn hex(tokenizer: &mut Tokenizer) -> Result<Token, ()>
{
    match tokenizer.ahead() {
        None => return Err(()),
        Some(c) => if !alpha(c) && !numeric(c) {
            return Err(());
        }
    }

    tokenizer.next();
    tokenizer.string.clear();
    tokenizer.read_while(|c| alpha(c) || numeric(c));
    let num = std::mem::take(&mut tokenizer.string);

    Ok(Token::Value(u16::from_str_radix(&num, 16).unwrap()))
}

fn num(tokenizer: &mut Tokenizer) -> Result<Token, ()>
{
    tokenizer.read_while(|c| numeric(c));
    let num = std::mem::take(&mut tokenizer.string);
    match num.parse::<u16>() {
        Err(_) => Err(()),
        Ok(value) => Ok(Token::Value(value))
    }
}

fn string(tokenizer: &mut Tokenizer) -> Result<Token, ()>
{
    if Some('\"') != tokenizer.ahead() {
        tokenizer.next();
        tokenizer.read_while(|c| c != '"');
    }

    let s = std::mem::take(&mut tokenizer.string);

    if Some('\"') != tokenizer.ahead() {
        return Err(());
    }
    tokenizer.next();
    Ok(Token::String(s))
}


fn ident(tokenizer: &mut Tokenizer) -> Result<Token, ()>
{
    tokenizer.read_while(|c| alpha(c) || c == '_');
    let ident = std::mem::take(&mut tokenizer.string);

    use Register::*;
    use Register16::*;
    use FlagRegister::*;
    use Operation::*;
    Ok(match ident.as_str() {
        "a" => A.into(),
        "b" => B.into(),
        "c" => C.into(),
        "d" => D.into(),
        "e" => E.into(),
        "h" => H.into(),
        "l" => L.into(),
        "z" => Z.into(),
        "n" => N.into(),
        "hc" => HC.into(),
        "cr" => CR.into(),
        "nz" => NZ.into(),
        "nc" => NC.into(),
        "af" => AF.into(),
        "bc" => BC.into(),
        "de" => DE.into(),
        "hl" => HL.into(),
        "sp" => SP.into(),
        "pc" => PC.into(),
        "add" => Add.into(),
        "and" => And.into(),
        "bit" => Bit.into(),
        "call" => Call.into(),
        "ccf" => Ccf.into(),
        "cp" => Cp.into(),
        "cpl" => Cpl.into(),
        "daa" => Daa.into(),
        "dec" => Dec.into(),
        "di" => Di.into(),
        "ei" => Ei.into(),
        "halt" => Halt.into(),
        "inc" => Inc.into(),
        "jp" => Jp.into(),
        "jr" => Jr.into(),
        "ld" => Ld.into(),
        "nop" => Nop.into(),
        "or" => Or.into(),
        "pop" => Pop.into(),
        "push" => Push.into(),
        "ret" => Ret.into(),
        "reti" => Reti.into(),
        "res" => Res.into(),
        "rl" => Rl.into(),
        "rla" => Rla.into(),
        "rlc" => Rlc.into(),
        "rlca" => Rlca.into(),
        "rr" => Rr.into(),
        "rra" => Rra.into(),
        "rrca" => Rrca.into(),
        "rst" => Rst.into(),
        "sbc" => Sbc.into(),
        "scf" => Scf.into(),
        "set" => Set.into(),
        "sla" => Sla.into(),
        "sra" => Sra.into(),
        "srl" => Srl.into(),
        "stop" => Stop.into(),
        "sub" => Sub.into(),
        "swap" => Swap.into(),
        "xor" => Xor.into(),
        _ => Token::Id(ident)
    })
}

fn direc(tokenizer: &mut Tokenizer) -> Result<Token, ()>
{
    tokenizer.read_while(|c| alpha(c) || numeric(c) || c == '_');
    let direc = std::mem::take(&mut tokenizer.string);

    use Directive::*;
    Ok(match direc.as_str() {
        ".ascii" => Ascii.into(),
        ".asciz" => Asciz.into(),
        ".byte"  => Byte.into(),
        ".data"  => Data.into(),
        ".fill"  => Fill.into(),
        ".org"   => Org.into(),
        ".text"  => Text.into(),
        ".use"   => Use.into(),
        ".utf8"  => Utf8.into(),
        "." => return Err(()),
        _ => Token::Id(direc)
    })
}

pub fn scan(input: &str) -> Result<Vec<Token>, ()>
{
    let chars: Vec<_> = input.chars().collect();
    let mut tokenizer = Tokenizer::new(chars);
    let mut tokens = Vec::new();

    while let Some(c) = tokenizer.read() {
        if c.is_whitespace() {
            if c == '\n' {
                match tokens.last() {
                    None | Some(&Token::Newline) => (),
                    _ => {
                        tokens.push(Token::Newline);
                    }
                }
            }
            tokenizer.next();
            continue;
        }

        match c {
            ';' => {
                while let Some(c) = tokenizer.next() {
                    if c == '\n' {
                        break;
                    }
                }
            },

            ',' => {
                tokens.push(Token::Comma);
            },

            ':' => {
                tokens.push(Token::Colon);
            },

            '.' => {
                let token = direc(&mut tokenizer)?;
                tokens.push(token);
            },

            '0' => {
                    tokens.push(if let Some(c) = tokenizer.ahead() {
                        match c {
                            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => octal(&mut tokenizer),
                            'b' => {
                                tokenizer.next();
                                binary(&mut tokenizer)?
                            },
                            'x' => {
                                tokenizer.next();
                                hex(&mut tokenizer)?
                            },
                            _ => Token::Value(0)
                        }
                    } else {
                        Token::Value(0)
                    });
            },

            '"' => {
                tokens.push(string(&mut tokenizer)?);
            },

            _ => {
                if alpha(c) || c == '_' {
                    let token = ident(&mut tokenizer)?;
                    tokens.push(token);
                } else if numeric(c) {
                    let token = num(&mut tokenizer)?;
                    tokens.push(token);
                } else {
                    return Err(());
                }
            }
        }

        tokenizer.next();
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token()
    {
        let input = r#"
            _start:
                ld a, 0x05
        "#;

        let tokens = scan(input).unwrap();
        println!("{:?}", tokens);
    }

    #[test]
    fn byte()
    {
        let input = r#"
            .byte 0x05
        "#;

        let tokens = scan(input).unwrap();
        println!("{:?}", tokens);
    }

    #[test]
    fn ascii()
    {
        let input = r#"
            .asciz "hello world"
        "#;

        let tokens = scan(input).unwrap();
        println!("{:?}", tokens);
    }

    #[test]
    fn label()
    {
        let input = r#"
            foo:
            .loop:
        "#;

        let tokens = scan(input).unwrap();
        println!("{:?}", tokens);
    }
}
