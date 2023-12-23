use crate::automaton::States::*;
use crate::automaton::token::{Position, Token, TokenClassification, TokenType};

mod token;


#[derive(PartialEq)]
enum States {
    Q1, // " "
    Q2, Q3, Q4, Q5, Q6, Q7, Q8, // program
    Q9, Q10, Q11, // var
    Q12, Q13, Q14, Q15, Q16, Q17, Q18, // integer
    Q19, // if
    Q20, Q21, Q22, Q23, Q24, // float
    Q25, Q26, Q27, Q28, Q29, Q30, Q31, // boolean
    Q32, Q33, Q34, Q35, Q36, Q37, // procedure
    Q38, Q39, Q40, Q41, // begin
    Q42, Q43, Q44, // end
    Q45, Q46, Q47, // else
    Q48, Q49, Q50, Q51, // then
    Q52, Q53, Q54, Q55, Q56, // while
    Q57, Q58, // do
    Q59, Q60, Q61, // not
    Q62, // { }
    Q63, // [a..z_0..9]
    Q64, // [0..9]
    Q65, Q66, Q67, Q68, Q69, Q70, Q71, Q72, Q73, // - . + ; * ( : ) =
    INVALID
}


pub struct Automaton {
    line: usize,
    column: usize,
    column_offset: usize,
    literal_buffer: String,
    current_state: States,
    end_states: Vec<States>,
    literal_ready_flag: bool
}


impl Automaton {
    fn new_line(&mut self) {
        self.line += 1;
        self.column = 0;
        self.column_offset = 0;
    }

    fn increase_column(&mut self) {
        self.column += 1;
    }

    fn get_current_line(&self) -> usize {
        self.line
    }

    fn get_current_row(&self) -> usize {
        self.column
    }

    fn transition(&mut self, character: char) {
        match self.current_state {
            Q1 => {
                match character {
                    ' ' => self.current_state = Q1,
                    '\n' => {
                        self.new_line();
                        self.current_state = Q1;
                    },
                    'p' => self.current_state = Q2,
                    'v' => self.current_state = Q9,
                    'i' => self.current_state = Q12,
                    'f' => self.current_state = Q20,
                    'b' => self.current_state = Q25,
                    'e' => self.current_state = Q42,
                    't' => self.current_state = Q48,
                    'w' => self.current_state = Q52,
                    'd' => self.current_state = Q57,
                    'n' => self.current_state = Q59,
                    _ => self.current_state = Q63
                }
            }
            Q2 => {
                if character == 'r' {
                    self.current_state = Q3;
                }
                else {self.current_state = Q63;}
            }
            Q3 => {
                if character == 'o' {
                    self.current_state = Q4;
                }
                else {self.current_state = Q63;}
            }
            Q4 => {
                if character == 'g' {
                    self.current_state = Q5;
                }
                else if character == 'c' {
                    self.current_state = Q32;
                }
                else {self.current_state = Q63;}
            }
            Q5 => {
                if character == 'r' {
                    self.current_state = Q6;
                }
                else {self.current_state = Q63;}
            }
            Q6 => {
                if character == 'a' {
                    self.current_state = Q7;
                }
                else {self.current_state = Q63;}
            }
            Q7 => {
                if character == 'm' {
                    self.current_state = Q8;
                }
                else {self.current_state = Q63;}
            }
            Q8 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
                else { self.current_state = INVALID }
            }
            Q9 => {
                if character == 'a' {
                    self.current_state = Q10;
                }
                else {self.current_state = Q63;}
            }
            Q10 => {
                if character == 'r' {
                    self.current_state = Q11;
                }
                else {self.current_state = Q63;}
            }
            Q11 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
                else { self.current_state = INVALID }
            }
            Q12 => {
                if character == 'n' {
                    self.current_state = Q13;
                }
                else if character == 'f' {
                    self.current_state = Q19;
                }
                else { self.current_state = Q63; }
            }
            Q13 => {
                if character == 't' {
                    self.current_state = Q14;
                }
                else { self.current_state = Q63; }
            }
            Q14 => {
                if character == 'e' {
                    self.current_state = Q15;
                }
                else { self.current_state = Q63; }
            }
            Q15 => {
                if character == 'g' {
                    self.current_state = Q16;
                }
                else { self.current_state = Q63; }
            }
            Q16 => {
                if character == 'e' {
                    self.current_state = Q17;
                }
                else { self.current_state = Q63; }
            }
            Q17 => {
                if character == 'r' {
                    self.current_state = Q18;
                }
                else { self.current_state = Q63; }
            }
            Q18 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
                else { self.current_state = INVALID }
            }
            Q19 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
            }
            Q20 => {
                if character == 'l' {
                    self.current_state = Q21;
                }
                else { self.current_state = Q63; }
            }
            Q21 => {
                if character == 'o' {
                    self.current_state = Q22;
                }
                else { self.current_state = Q63; }
            }
            Q22 => {
                if character == 'a' {
                    self.current_state = Q23;
                }
                else { self.current_state = Q63; }
            }
            Q23 => {
                if character == 't' {
                    self.current_state = Q24;
                }
                else { self.current_state = Q63; }
            }
            Q24 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
                else { self.current_state = INVALID }
            }
            Q25 => {
                if character == 'o' {
                    self.current_state = Q26;
                }
                else if character == 'e' {
                    self.current_state = Q38;
                }
                else { self.current_state = Q63; }
            }
            Q26 => {
                if character == 'o' {
                    self.current_state = Q27;
                }
                else { self.current_state = Q63; }
            }
            Q27 => {
                if character == 'l' {
                    self.current_state = Q28;
                }
                else { self.current_state = Q63; }
            }
            Q28 => {
                if character == 'e' {
                    self.current_state = Q29;
                }
                else { self.current_state = Q63; }
            }
            Q29 => {
                if character == 'a' {
                    self.current_state = Q30;
                }
                else { self.current_state = Q63; }
            }
            Q30 => {
                if character == 'n' {
                    self.current_state = Q31;
                }
                else { self.current_state = Q63; }
            }
            Q31 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
                else { self.current_state = INVALID; }
            }
            Q32 => {
                if character == 'e' {
                    self.current_state = Q33;
                }
                else { self.current_state = Q63; }
            }
            Q33 => {
                if character == 'd' {
                    self.current_state = Q34;
                }
                else { self.current_state = Q63; }
            }
            Q34 => {
                if character == 'u' {
                    self.current_state = Q35;
                }
                else { self.current_state = Q63; }
            }
            Q35 => {
                if character == 'r' {
                    self.current_state = Q36;
                }
                else { self.current_state = Q63; }
            }
            Q36 => {
                if character == 'e' {
                    self.current_state = Q37;
                }
                else { self.current_state = Q63; }
            }
            Q37 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
                else { self.current_state = INVALID; }
            }
            Q38 => {
                if character == 'g' {
                    self.current_state = Q39;
                }
                else { self.current_state = Q63; }
            }
            Q39 => {
                if character == 'i' {
                    self.current_state = Q40;
                }
                else { self.current_state = Q63; }
            }
            Q40 => {
                if character == 'n' {
                    self.current_state = Q41;
                }
                else { self.current_state = Q63; }
            }
            Q41 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
                else { self.current_state = INVALID; }
            }
            Q42 => {
                if character == 'n' {
                    self.current_state = Q43;
                }
                else if character == 'l' {
                    self.current_state = Q45;
                }
                else { self.current_state = Q63; }
            }
            Q43 => {
                if character == 'd' {
                    self.current_state = Q44;
                }
                else { self.current_state = Q63; }
            }
            Q44 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
                else { self.current_state = INVALID; }
            }
            Q45 => {
                if character == 's' {
                    self.current_state = Q46;
                }
                else { self.current_state = Q63; }
            }
            Q46 => {
                if character == 'e' {
                    self.current_state = Q47;
                }
                else { self.current_state = Q63; }
            }
            Q47 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
                else { self.current_state = INVALID; }
            }
            Q48 => {
                if character == 'h' {
                    self.current_state = Q49;
                }
                else { self.current_state = Q63; }
            }
            Q49 => {
                if character == 'e' {
                    self.current_state = Q50;
                }
                else { self.current_state = Q63; }
            }
            Q50 => {
                if character == 'n' {
                    self.current_state = Q51;
                }
                else { self.current_state = Q63; }
            }
            Q51 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
                else { self.current_state = INVALID; }
            }
            Q52 => {
                if character == 'h' {
                    self.current_state = Q53;
                }
                else { self.current_state = Q63; }
            }
            Q53 => {
                if character == 'i' {
                    self.current_state = Q54;
                }
                else { self.current_state = Q63; }
            }
            Q54 => {
                if character == 'l' {
                    self.current_state = Q55;
                }
                else { self.current_state = Q63; }
            }
            Q55 => {
                if character == 'e' {
                    self.current_state = Q56;
                }
                else { self.current_state = Q63; }
            }
            Q56 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
                else { self.current_state = INVALID; }
            }
            Q57 => {
                if character == 'o' {
                    self.current_state = Q58;
                }
                else { self.current_state = Q63; }
            }
            Q58 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
                else { self.current_state = Q63; }
            }
            Q59 => {
                if character == 'o' {
                    self.current_state = Q60;
                }
                else { self.current_state = Q63; }
            }
            Q60 => {
                if character == 't' {
                    self.current_state = Q61;
                }
                else { self.current_state = Q63; }
            }
            Q61 => {
                if character == ' ' {
                    self.current_state = Q1;
                }
                else { self.current_state = Q63; }
            }
            Q62 => {}
            Q63 => {
                let a = character.is_alphanumeric();
                if character.is_alphanumeric() {
                    self.current_state = Q63;
                }
                else if character == '_' {
                    self.current_state = Q63;
                }
                else {
                    self.literal_ready_flag = true;
                    self.current_state = Q1;
                }
            }
            Q64 => {}
            Q65 => {
                self.current_state = Q1;
            }
            Q66 => {
                self.current_state = Q1;
            }
            Q67 => {
                self.current_state = Q1;
            }
            Q68 => {
                self.current_state = Q1;
            }
            Q69 => {
                self.current_state = Q1;
            }
            Q70 => {
                self.current_state = Q1;
            }
            Q71 => {
                if character == '=' {
                    self.current_state = Q73;
                }
                else { self.current_state = Q1; }
            }
            Q72 => {
                self.current_state = Q1;
            }
            Q73 => {
                self.current_state = Q1;
            }
            INVALID => {}
        }
    }

    fn generate_token(&self) -> Token {
        if self.current_state == Q8 {
            Token::new(TokenType::Program, TokenClassification::Keywords, self.line, self.column_offset)
        }
        else if self.current_state == Q11 {
            Token::new(TokenType::Var, TokenClassification::Keywords, self.line, self.column)
        }
        else if self.current_state == Q18 {
            Token::new(TokenType::Integer, TokenClassification::Keywords, self.line, self.column)
        }
        else if self.current_state == Q19 {
            Token::new(TokenType::If, TokenClassification::Keywords, self.line, self.column)
        }
        else if self.current_state == Q24 {
            Token::new(TokenType::Float, TokenClassification::Keywords, self.line, self.column)
        }
        else if self.current_state == Q31 {
            Token::new(TokenType::Boolean, TokenClassification::Keywords, self.line, self.column)
        }
        else if self.current_state == Q37 {
            Token::new(TokenType::Procedure, TokenClassification::Keywords, self.line, self.column)
        }
        else if self.current_state == Q41 {
            Token::new(TokenType::Begin, TokenClassification::Keywords, self.line, self.column)
        }
        else if self.current_state == Q44 {
            Token::new(TokenType::End, TokenClassification::Keywords, self.line, self.column)
        }
        else if self.current_state == Q47 {
            Token::new(TokenType::Else, TokenClassification::Keywords, self.line, self.column)
        }
        else if self.current_state == Q51 {
            Token::new(TokenType::Then, TokenClassification::Keywords, self.line, self.column)
        }
        else if self.current_state == Q56 {
            Token::new(TokenType::While, TokenClassification::Keywords, self.line, self.column)
        }
        else if self.current_state == Q58 {
            Token::new(TokenType::Do, TokenClassification::Keywords, self.line, self.column)
        }
        else if self.current_state == Q61 {
            Token::new(TokenType::Not, TokenClassification::Keywords, self.line, self.column)
        }
        else if self.current_state == Q63 {
            Token::new(TokenType::Identifier(self.literal_buffer.to_string()), TokenClassification::Identifiers, self.line, self.column)
        }
        else { panic!("Invalid token.") }
    }
}


impl Automaton {
    pub fn new() -> Self {
        Automaton {
            line: 0,
            column: 0,
            column_offset: 0,
            literal_buffer: "".to_string(),
            current_state: Q1,
            end_states: Vec::from([Q8, Q11, Q18, Q19, Q24, Q31, Q37, Q41, Q44, Q47, Q51, Q56, Q58, Q61, Q63, Q64, Q65, Q66, Q67, Q68, Q69, Q70, Q71, Q72, Q73]),
            literal_ready_flag: false,
        }
    }

    pub fn process_input(&mut self, input: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let static_input = input.clone();


        for character in input.chars() {
            self.transition(character);

            if self.end_states.contains(&self.current_state) {
                self.column_offset += 1;

                if self.current_state == Q63 && self.literal_ready_flag {
                    self.literal_buffer = static_input[self.column-self.column_offset-1..self.column-1].to_string();
                    tokens.push(self.generate_token());
                    self.literal_ready_flag = false;
                    self.column_offset = 0;
                    continue;
                }
                else if (self.current_state != Q63) {
                    tokens.push(self.generate_token());
                    self.current_state = Q1;
                    self.column_offset = 0;
                    continue;
                }
            }

            self.increase_column();
        }

        tokens
    }
}


#[cfg(test)]
mod tests {
    use crate::automaton::Automaton;
    use crate::automaton::token::{Token, TokenClassification, TokenType};

    #[test]
    fn reads_program() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("program".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::Program, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn reads_var() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("var".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::Var, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn reads_integer() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("integer".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::Integer, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn reads_if() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("if".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::If, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn reads_float() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("float".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::Float, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn reads_boolean() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("boolean".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::Boolean, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn reads_begin() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("begin".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::Begin, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn reads_end() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("end".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::End, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn reads_else() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("else".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::Else, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn reads_then() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("then".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::Then, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn reads_while() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("while".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::While, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn reads_do() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("do".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::Do, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn reads_not() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("not".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::Not, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn reads_procedure() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("procedure".to_string());
        assert_eq!(tokens.get(0).unwrap().token, Token::new(TokenType::Procedure, TokenClassification::Keywords, 0, 0).token);
    }

    #[test]
    fn test_subject() {
        let mut automaton = Automaton::new();
        let input = "program teste; {programa exemplo}\nvar\n\tvalor: integer;\n\tvalor2: float;\nbegin\n\tvalor1 := 10;\nend";
        let tokens = automaton.process_input(input.to_string());
    }
}