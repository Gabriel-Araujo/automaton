use std::fmt::{Display, Formatter, Result};
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

    pub fn get_token_name(&self) -> String {
        self.token.clone()
    }

    pub fn get_token_type(&self) -> String {
        self.token_type.to_string()
    }
}


impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TokenType::Keyword => {write!(f, "Keyword")}
            TokenType::Delimiter => {write!(f, "Delimiter")}
            TokenType::Assignment => {write!(f, "Assignment")}
            TokenType::Comparison => {write!(f, "Comparison")}
            TokenType::Arithmetic => {write!(f, "Arithmetic")}
            TokenType::Logic => {write!(f, "Logic")}
            TokenType::Identifier => {write!(f, "Identifier")}
            TokenType::IntegerNumber => {write!(f, "IntegerNumber")}
            TokenType::FloatNumber => {write!(f, "FloatNumber")}
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::lexical_parser::token::TokenType;

    #[test]
    fn to_string_test() {
        let a = TokenType::Keyword;
        assert_eq!(a.to_string(), "Keyword")
    }
}