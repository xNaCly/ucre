#[derive(Debug)]
pub enum TokenType {
    Ident(String),
    String(String),
    Number(f64),
    Lua(String),
    CurlyLeft,
    CurlyRight,
    Eof,
}
