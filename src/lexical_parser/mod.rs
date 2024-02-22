mod token;

use std::process::exit;
use crate::lexical_parser::States::*;
use crate::lexical_parser::token::{Lexeme, TokenType};

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
    Q64, Q65, Q66, Q67, Q68, Q69, Q70, Q71, Q72, // := - + . ; * ( : )
    Q73, Q74, // = <
    Q75, Q76, // Integer Number, Float Number [0..9]
    Q77, Q78, Q79, Q80, // <> > <= >=
    Q81, // /
    Q82, Q83, Q84, // And
    Q85, Q86, // Or
    Q87, // ,
    INVALID
}

pub struct Automaton {
    line: usize,
    position: usize,
    buffer: String,
    current_state: States,
    tokens: Vec<Lexeme>,
    input: String,
}

impl Automaton {
    fn new_line(&mut self) {
        self.line += 1;
        self.position = 0;
    }

    fn peek(&self) -> char {
        match self.input.chars().nth(self.position+1) {
            None => ' ',
            Some(value) => {value}
        }
    }

    fn next_char_is_delimiter(&mut self, character: char) {
        self.tokens.push(self.generate_token());
        if character == ';' {
            self.current_state = Q68;
        }
        else if character == '(' {
            self.current_state = Q70;
        }
        else if character == ':' {
            self.current_state = Q71;
        }
        else if character == ')' {
            self.current_state = Q72;
        }
        else if character == ',' {
            self.current_state = Q87;
        }
        else if character == '.' {
            self.current_state = Q67;
        }
        else { self.current_state = Q1 }

        self.tokens.push(self.generate_token());
        self.current_state = Q1;
    }

    fn next_char_is_arithmetic(&mut self, character: char) {
        self.tokens.push(self.generate_token());
        if character == '+' {
            self.current_state = Q66;
        }
        else if character == '-' {
            self.current_state = Q65;
        }
        else if character == '/' {
            self.current_state = Q81;
        }
        else if character == '*' {
            self.current_state = Q69;
        }
        else { self.current_state = Q1 }

        self.tokens.push(self.generate_token());
        self.current_state = Q1;
    }

    fn is_delimiter(&self, character: char) -> bool {
        let delimiters = [';', ':', '.', ',', '(', ')'];

        delimiters.contains(&character)
    }

    fn is_arithmetic(&self, character: char) -> bool {
        let delimiters = ['+', '-', '/', '*'];

        delimiters.contains(&character)
    }

    fn increase_position(&mut self) {
        self.position += 1;
    }

    fn transition(&mut self, character: char) {
        match self.current_state {
            Q1 => {
                self.buffer = "".to_string();
                match character {
                    ' ' => self.current_state = Q1,
                    '\n' => {
                        self.new_line();
                        self.current_state = Q1;
                    },
                    '\r' => {
                        self.current_state = Q1;
                    },
                    '\t' => self.current_state = Q1,
                    'p' => self.current_state = Q2,
                    'v' => self.current_state = Q9,
                    'i' => self.current_state = Q12,
                    'r' => self.current_state = Q20,
                    'b' => self.current_state = Q25,
                    'e' => self.current_state = Q42,
                    't' => self.current_state = Q48,
                    'w' => self.current_state = Q52,
                    'd' => self.current_state = Q57,
                    'n' => self.current_state = Q59,
                    '{' => self.current_state = Q62,
                    char if char.is_alphabetic() => self.current_state = Q63,
                    '-' => self.current_state = Q65,
                    '+' => self.current_state = Q66,
                    '.' => self.current_state = Q67,
                    ';' => self.current_state = Q68,
                    '*' => self.current_state = Q69,
                    '(' => self.current_state = Q70,
                    ':' => self.current_state = Q71,
                    ')' => self.current_state = Q72,
                    '=' => self.current_state = Q73,
                    '<' => self.current_state = Q74,
                    char if char.is_numeric() => self.current_state = Q75,
                    '>' => self.current_state = Q78,
                    '/' => self.current_state = Q81,
                    'a' => self.current_state = Q82,
                    'o' => self.current_state = Q85,
                    ',' => self.current_state = Q87,
                    _ => self.current_state = INVALID
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
                    self.tokens.push(self.generate_token());
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
                if character == ' ' || character == '\r' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character == '\n' {
                    self.tokens.push(self.generate_token());
                    self.new_line();
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
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else { self.next_char_is_delimiter(character) }
            }
            Q19 => {
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
            }
            Q20 => {
                if character == 'e' {
                    self.current_state = Q21;
                }
                else { self.current_state = Q63; }
            }
            Q21 => {
                if character == 'a' {
                    self.current_state = Q23;
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
                if character == 'l' {
                    self.current_state = Q24;
                }
                else { self.current_state = Q63; }
            }
            Q24 => {
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else { self.next_char_is_delimiter(character) }
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
                    self.tokens.push(self.generate_token());
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
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character == '\n' || character == '\r' {
                    self.tokens.push(self.generate_token());
                    self.new_line();
                    self.current_state = Q1;
                }
                else { self.current_state = Q63; }
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
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character == '\n' || character == '\r' {
                    self.tokens.push(self.generate_token());
                    self.new_line();
                    self.current_state = Q1;
                }
                else { self.current_state = Q63; }
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
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character == '\n' || character == '\r' {
                    self.tokens.push(self.generate_token());
                    self.new_line();
                    self.current_state = Q1;
                }
                else if self.is_delimiter(character) {
                    self.next_char_is_delimiter(character);
                }
                else { self.current_state = Q63; }
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
                    self.tokens.push(self.generate_token());
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
                if character == ' ' || character == '\n' || character == '\r' {
                    self.tokens.push(self.generate_token());
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
                    self.tokens.push(self.generate_token());
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
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character == '\n' || character == '\r' {
                    self.tokens.push(self.generate_token());
                    self.new_line();
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
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else { self.current_state = Q63; }
            }
            Q62 => { // comentarios
                    if character == '}' {
                        self.current_state = Q1;
                    }
                    else if character == '\n' || character == '\r' {
                        self.current_state = INVALID;
                    }
            }
            Q63 => { // Identificador
                if character.is_alphanumeric() {
                    self.current_state = Q63;
                }
                else if character == '_' {
                    self.current_state = Q63;
                }
                else if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character == '\n' || character == '\r' {
                    self.tokens.push(self.generate_token());
                    self.new_line();
                    self.current_state = Q1;
                }
                else if character ==':' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q71;
                }
                else if self.is_delimiter(character) {
                    self.next_char_is_delimiter(character);
                }
                else if self.is_arithmetic(character) {
                    self.next_char_is_arithmetic(character);
                }
            }
            Q64 => { // :=
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character == ';' || character == '\n' || character == '\r' {
                    self.current_state = INVALID;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
            }
            Q65 => { // -
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
                else {
                    self.current_state = INVALID;
                }
            }
            Q66 => { // +
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
                else {
                    self.current_state = INVALID;
                }
            }
            Q67 => { // .
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                if character == '\n' || character == '\r' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
                else {
                    self.current_state = INVALID;
                }
            }
            Q68 => { // ;
                self.tokens.push(self.generate_token());
                self.current_state = Q1;
                if character == '\n' || character == '\r' {
                    self.new_line();
                }
            }
            Q69 => { // *
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
                else {
                    self.current_state = INVALID;
                }
            }
            Q70 => { // (
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
                else {
                    self.current_state = INVALID;
                }
            }
            Q71 => { // :
                if character == '=' {
                    self.current_state = Q64;
                }
                else if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character == 'i' {
                    self.current_state = Q12;
                }
                else if character == 'r' {
                    self.current_state = Q20;
                }
                else {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
            }
            Q72 => { // )
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
                else {
                    self.current_state = Q1;
                }
            }
            Q73 => { // =
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
                else {
                    self.current_state = INVALID;
                }
            }
            Q74 => { // <
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character == '=' {
                    self.current_state = Q79;
                }
                else if character == '>' {
                    self.current_state = Q77;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
                else {
                    self.current_state = Q1;
                }
            }
            Q75 => { // Numeros Inteiros
                if character.is_numeric() {
                    self.current_state = Q75;
                }
                else if character == '.' {
                    self.current_state = Q76;
                }
                else if character == ';' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q68;
                }
                else  if character == ' ' || character == '\n' || character == '\r' || character == ')'{
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else {
                    self.current_state = INVALID;
                }
            }
            Q76 => { // Numeros Reais
                if character.is_numeric() {
                    self.current_state = Q76;
                }
                else if character == '.' {
                    self.current_state = INVALID;
                }
                else if character == ';' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else  if character == ' '{
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else {
                    self.current_state = INVALID;
                }
            }
            Q77 => { // <>
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
                else {
                    self.current_state = INVALID;
                }
            }
            Q78 => { // >
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character == '=' {
                    self.current_state = Q80;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
                else {
                    self.current_state = INVALID;
                }
            }
            Q79 => { // <=
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
                else {
                    self.current_state = INVALID;
                }
            }
            Q80 => { // >=
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
                else {
                    self.current_state = INVALID;
                }
            }
            Q81 => { // /
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character.is_numeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q75;
                }
                else if character.is_alphanumeric() {
                    self.tokens.push(self.generate_token());
                    self.buffer = "".to_string();
                    self.current_state = Q63;
                }
                else {
                    self.current_state = INVALID;
                }
            }
            Q82 => {
                if character == 'n' {
                    self.current_state = Q83;
                }
                else { self.current_state = Q63; }
            }
            Q83 => {
                if character == 'd' {
                    self.current_state = Q84;
                }
                else { self.current_state = Q63; }
            }
            Q84 => {
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character == '\n' || character == '\r' {
                    self.tokens.push(self.generate_token());
                    self.new_line();
                    self.current_state = Q1;
                }
                else { self.current_state = Q63 }
            }
            Q85 => {
                if character == 'r' {
                    self.current_state = Q86;
                }
                else { self.current_state = Q63; }
            }
            Q86 => {
                if character == ' ' {
                    self.tokens.push(self.generate_token());
                    self.current_state = Q1;
                }
                else if character == '\n' || character == '\r' {
                    self.tokens.push(self.generate_token());
                    self.new_line();
                    self.current_state = Q1;
                }
                else { self.current_state = Q63 }
            }
            Q87 => {
                self.tokens.push(self.generate_token());
                self.current_state = Q1;
            }
            INVALID => {}
        }
    }

    fn generate_token(&self) -> Lexeme {
        if self.current_state == Q8 {
            Lexeme::new("Program".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q11 {
            Lexeme::new("Var".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q18 {
            Lexeme::new("Integer".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q19 {
            Lexeme::new("If".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q24 {
            Lexeme::new("Float".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q31 {
            Lexeme::new("Boolean".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q37 {
            Lexeme::new("Procedure".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q41 {
            Lexeme::new("Begin".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q44 {
            Lexeme::new("End".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q47 {
            Lexeme::new("Else".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q51 {
            Lexeme::new("Then".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q56 {
            Lexeme::new("While".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q58 {
            Lexeme::new("Do".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q61 {
            Lexeme::new("Not".to_string(), TokenType::Keyword, self.line)
        }
        else if self.current_state == Q63 {
            Lexeme::new(self.buffer.trim().to_string(), TokenType::Identifier, self.line)
        }
        else if self.current_state == Q64 {
            Lexeme::new(":=".to_string(), TokenType::Assignment, self.line)
        }
        else if self.current_state == Q65 {
            Lexeme::new("-".to_string(), TokenType::Arithmetic, self.line)
        }
        else if self.current_state == Q66 {
            Lexeme::new("+".to_string(), TokenType::Arithmetic, self.line)
        }
        else if self.current_state == Q67 {
            Lexeme::new(".".to_string(), TokenType::Delimiter, self.line)
        }
        else if self.current_state == Q68 {
            Lexeme::new(";".to_string(), TokenType::Delimiter, self.line)
        }
        else if self.current_state == Q69 {
            Lexeme::new("*".to_string(), TokenType::Arithmetic, self.line)
        }
        else if self.current_state == Q70 {
            Lexeme::new("(".to_string(), TokenType::Delimiter, self.line)
        }
        else if self.current_state == Q71 {
            Lexeme::new(":".to_string(), TokenType::Delimiter, self.line)
        }
        else if self.current_state == Q72 {
            Lexeme::new(")".to_string(), TokenType::Delimiter, self.line)
        }
        else if self.current_state == Q73 {
            Lexeme::new("=".to_string(), TokenType::Comparison, self.line)
        }
        else if self.current_state == Q74 {
            Lexeme::new("<".to_string(), TokenType::Comparison, self.line)
        }
        else if self.current_state == Q75 {
            Lexeme::new(self.buffer.trim().to_string(), TokenType::IntegerNumber, self.line)
        }
        else if self.current_state == Q76 {
            Lexeme::new(self.buffer.trim().to_string(), TokenType::FloatNumber, self.line)
        }
        else if self.current_state == Q77 {
            Lexeme::new("<>".to_string(), TokenType::Comparison, self.line)
        }
        else if self.current_state == Q78 {
            Lexeme::new(">".to_string(), TokenType::Comparison, self.line)
        }
        else if self.current_state == Q79 {
            Lexeme::new("<=".to_string(), TokenType::Comparison, self.line)
        }
        else if self.current_state == Q80 {
            Lexeme::new(">=".to_string(), TokenType::Comparison, self.line)
        }
        else if self.current_state == Q81 {
            Lexeme::new("/".to_string(), TokenType::Arithmetic, self.line)
        }
        else if self.current_state == Q84 {
            Lexeme::new("And".to_string(), TokenType::Logic, self.line)
        }
        else if self.current_state == Q86 {
            Lexeme::new("Or".to_string(), TokenType::Logic, self.line)
        }
        else if self.current_state == Q87 {
            Lexeme::new(",".to_string(), TokenType::Delimiter, self.line)
        }
        else {
            eprintln!("Last value on buffer: {}", self.buffer);
            eprintln!("Invalid character at line {} and column {}", self.line, self.position);
            exit(1);
        }
    }
}


impl Automaton {
    pub fn new() -> Self {
        Automaton {
            line: 1,
            position: 0,
            buffer: "".to_string(),
            current_state: Q1,
            tokens: Vec::new(),
            input: "".to_string(),
        }
    }

    pub fn process_input(&mut self, input: String) -> Vec<Lexeme> {
        let input_as_chars = input.chars();
        self.input = input.clone();

        for character in input_as_chars {
            self.increase_position();
            self.transition(character);
            self.buffer.push(character);
            if self.current_state == INVALID {
                eprintln!("Last value on buffer: {}", self.buffer);
                eprintln!("Invalid character at line {} and column {}", self.line, self.position);
                exit(1);
            }
        }
        if self.current_state != Q1 {
            self.tokens.push(self.generate_token());
        }

        self.tokens.clone()
    }
}


#[cfg(test)]
mod tests {
    use crate::lexical_parser::Automaton;
    use crate::lexical_parser::token::{Lexeme, TokenType};

    #[test]
    fn reads_program() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("program ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("Program".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_var() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("var ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("Var".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_integer() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("integer ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("Integer".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_if() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("if ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("If".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_float() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("float ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("Float".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_boolean() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("boolean ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("Boolean".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_begin() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("begin ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("Begin".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_end() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("end ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("End".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_else() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("else ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("Else".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_then() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("then ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("Then".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_while() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("while ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("While".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_do() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("do ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("Do".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_not() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("not ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("Not".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_procedure() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("procedure ".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("Procedure".to_string(), TokenType::Keyword, 0));
    }

    #[test]
    fn reads_identifier() {
        let mut automaton = Automaton::new();
        let tokens = automaton.process_input("Hello_world".to_string());
        assert_eq!(*tokens.get(0).unwrap(), Lexeme::new("Hello_world".to_string(), TokenType::Identifier, 0));
    }

    #[test]
    fn test_subject() {
        let mut automaton = Automaton::new();
        let input = "program teste; {programa exemplo}\nvar\n\tvalor: integer;\n\tvalor2: float;\nbegin\n\tvalor1 := 10;\nend";
        // let tokens = automaton.process_input(input.to_string());
    }
}