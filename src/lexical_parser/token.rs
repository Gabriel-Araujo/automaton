#[derive(PartialEq, Debug, Clone)]
pub struct Lexeme {
    token: String,
    token_type: TokenType,
    line: usize
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Keyword, // Program, Var, Integer, Float, Boolean, Procedure, Begin, End, If, Then, Else, While, Do, Not,
    Delimiter, // ; : . , ( )
    Assignment, // :=
    Comparison, // = < > <= >= <>
    Arithmetic, // + - / *
    Logic, // And Or,
    Identifier,
    IntegerNumber,
    FloatNumber
}


impl Lexeme {
    pub fn new(token: String, token_type: TokenType, line: usize) -> Self {
        Lexeme {token, token_type, line}
    }
}