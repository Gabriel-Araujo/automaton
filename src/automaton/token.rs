
#[derive(PartialEq, Debug)]
pub struct Token {
    pub token: TokenType,
    pub classification: TokenClassification,
    pub position: Position
}


#[derive(PartialEq, Debug)]
pub enum TokenType {
    // Palavras chaves
    Program, Var, Integer, Float, Boolean, Procedure, Begin, End, If, Then, Else, While, Do, Not,

    // Delimitadores
    Semicolon, Dot, Colon, LeftParentheses, RightParentheses, Comma,

    // Atribuição
    ColonEqual,

    // Relacionais
    Equal, Less, More, LessEqual, MoreEqual, LessMore,

    // Matematicos
    Add, Subtract, Multiply, Divide,

    And, Or, Identifier(String)

}

#[derive(PartialEq, Debug)]
pub enum TokenClassification {
    Keywords, Identifiers, Integer, Float, Delimiters, Assignment, Relational
}


#[derive(PartialEq, Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize
}



impl Token {
    pub fn new(token: TokenType, classification: TokenClassification, line: usize, column: usize) -> Token {
        let position = Position {line, column};
        Token {
            token,
            classification,
            position
        }
    }
}