use shared::types::{token::TokenType, uom::UcreError};

pub struct Lexer<'lexer> {
    source: &'lexer [u8],
    pos: usize,
    line: usize,
}

impl<'lexer> Lexer<'lexer> {
    pub fn new(i: &'lexer [u8]) -> Self {
        Lexer {
            source: i,
            pos: 0,
            line: 1,
        }
    }

    pub fn run(&mut self) -> Result<Vec<TokenType>, UcreError> {
        let mut r = vec![];
        while !self.is_eof() {
            let t;
            self.skip_whitespace()?;
            if self.is_eof() {
                break;
            }
            match self.cur()? {
                '{' => t = TokenType::CurlyLeft,
                '}' => t = TokenType::CurlyRight,
                '"' => {
                    self.advance();
                    let start = self.pos;
                    while !self.is_eof() && self.cur()? != '"' {
                        self.advance();
                    }
                    if self.is_eof() {
                        return Err(UcreError::new(format!(
                            "ucc: Unterminated String in line {}",
                            self.line - 1
                        )));
                    }
                    let bytes = self.source.get(start..self.pos).ok_or_else(|| {
                        UcreError::from_str(
                            "ucc: Failed to extract bytes for a string from the input",
                        )
                    })?;
                    t = TokenType::String(String::from_utf8(bytes.to_vec()).map_err(|_| {
                        UcreError::from_str("ucc: Failed to convert a byte vector into a String")
                    })?)
                }
                '/' if self.next()? == '/' => {
                    while !self.is_eof() && self.cur()? != '\n' {
                        self.advance();
                    }
                    continue;
                }
                '0'..='9' => {
                    let start = self.pos;
                    self.advance();
                    while !self.is_eof()
                        && matches!(self.cur()?, '0'..='9' | '_' | '.' | 'A'..='F' | 'a'..='f')
                    {
                        self.advance();
                    }
                    let bytes = self.source.get(start..self.pos).ok_or_else(|| {
                        UcreError::from_str(
                            "ucc: Failed to extract bytes for a string from the input",
                        )
                    })?;
                    let number = String::from_utf8(bytes.to_vec())
                        .map_err(|_| {
                            UcreError::from_str(
                                "ucc: Failed to convert a byte vector into a String",
                            )
                        })?
                        .parse::<f64>()
                        .map_err(|e| UcreError::new(e.to_string()))?;
                    t = TokenType::Number(number);
                }
                'a'..='z' | 'A'..='Z' => {
                    let start = self.pos;
                    self.advance();
                    while !self.is_eof()
                        && matches!(self.cur()?, '-' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9')
                    {
                        self.advance();
                    }
                    let bytes = self.source.get(start..self.pos).ok_or_else(|| {
                        UcreError::from_str(
                            "ucc: Failed to extract bytes for a string from the input",
                        )
                    })?;
                    let ident = String::from_utf8(bytes.to_vec()).map_err(|_| {
                        UcreError::from_str("ucc: Failed to convert a byte vector into a String")
                    })?;
                    t = if ident == "lua" {
                        self.skip_whitespace()?;
                        if self.cur()? != '[' {
                            return Err(UcreError::from_str(
                                "ucc: Invalid lua block, missing [ to signal start",
                            ));
                        }
                        self.advance();
                        self.skip_whitespace()?;
                        let start = self.pos;
                        while !self.is_eof() && self.cur()? != ']' {
                            self.advance();
                        }
                        let bytes = self.source.get(start..self.pos - 1).ok_or_else(|| {
                            UcreError::from_str(
                                "ucc: Failed to extract bytes for a string from the input",
                            )
                        })?;
                        let ident = String::from_utf8(bytes.to_vec()).map_err(|_| {
                            UcreError::from_str(
                                "ucc: Failed to convert a byte vector into a String",
                            )
                        })?;

                        TokenType::Lua(ident)
                    } else {
                        TokenType::Ident(ident)
                    }
                }
                c @ _ => {
                    return Err(UcreError::new(format!(
                        "ucc: Unknown character '{c}' in line {}",
                        self.line - 1
                    )))
                }
            }
            r.push(t);
            self.advance();
        }
        r.push(TokenType::Eof);
        Ok(r)
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn skip_whitespace(&mut self) -> Result<(), UcreError> {
        while !self.is_eof() {
            let cur = self.cur()?;
            if !cur.is_whitespace() {
                break;
            } else if cur == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        Ok(())
    }

    fn cur(&self) -> Result<char, UcreError> {
        self.source
            .get(self.pos)
            .ok_or_else(|| {
                dbg!(UcreError::from_str(
                    "ucc: can't access current character, unexpected EOF"
                ))
            })
            .map(|x| *x as char)
    }

    fn next(&self) -> Result<char, UcreError> {
        self.source
            .get(self.pos + 1)
            .ok_or_else(|| UcreError::from_str("ucc: can't access next character, unexpected EOF"))
            .map(|x| *x as char)
    }

    fn advance(&mut self) {
        self.pos += 1
    }
}
