use std::collections::HashMap;
use crate::lexer::tokens::*;

pub fn run(data: String) -> Option<Vec<Token>> {
    let keywords_map = create_keyword_map();
    let mut tokens = Vec::new();

    if data.trim().is_empty() {
        return None;
    }
    
    let mut buffer_word = String::new();
    let mut data_as_peekable = data.trim().chars().peekable();
    while let Some(&ch) = data_as_peekable.peek() {
        if ch.is_whitespace() {
            data_as_peekable.next();
        }
        else if ch.is_alphabetic() {
            buffer_word.clear();
            while let Some(&ch) = data_as_peekable.peek() {
                if ch.is_alphabetic() {
                    buffer_word.push(ch);
                    data_as_peekable.next();
                } else {
                    break;
                }
            }
            if let Some(token) = process_word(&buffer_word, &keywords_map) {
                tokens.push(token);
            }
            else {
                tokens.push(Token::new(TokenType::Ident(buffer_word.clone())))
            }
        }
        else if ch.is_numeric() || ch == '-' {
            let mut buffer_number = String::new();
            if ch == '-' {
                buffer_number.push(ch);
                data_as_peekable.next();
            }
            while let Some(&ch) = data_as_peekable.peek() {
                if ch.is_numeric() {
                    buffer_number.push(ch);
                    data_as_peekable.next();
                } else {
                    break;
                }
            }
            if let Ok(num) = buffer_number.parse::<i32>() {
                tokens.push(Token::new(TokenType::IntLit(num as isize)));
            }
        } else {
            buffer_word.clear();
            buffer_word.push(ch);
            if let Some(token) = process_word(&buffer_word, &keywords_map) {
                tokens.push(token);
            }
            data_as_peekable.next();
        }
    }
    Some(tokens)
}

fn process_word(word: &str, keywords_map: &HashMap<String, TokenType>) -> Option<Token> {
    keywords_map.get(word).cloned().map(|token_type| Token::new(token_type))
}

fn create_keyword_map() -> HashMap<String, TokenType> {
    let mut map = HashMap::new();

    map.insert(String::from("return"), TokenType::Ret);
    map.insert(String::from("("), TokenType::OpenParen);
    map.insert(String::from(")"), TokenType::ClosedParen);
    map.insert(String::from("var"), TokenType::Var);
    map.insert(String::from("="), TokenType::Eq);
    
    map
}