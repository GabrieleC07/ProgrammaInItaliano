#[cfg(test)]
mod tests {
    use crate::lexer::tokens::{Token, TokenType};
    use crate::lexer::lexer;

    #[test]
    fn lexer_return_multiple_number_positive() {
        assert_eq!(lexer::run(String::from("return(91)")).unwrap(), vec![
            Token::new(TokenType::Ret),
            Token::new(TokenType::OpenParen),            
            Token::new(TokenType::IntLit(91)),
            Token::new(TokenType::ClosedParen),
        ]);

        assert_eq!(lexer::run(String::from("return(101)")).unwrap(), vec![
            Token::new(TokenType::Ret),
            Token::new(TokenType::OpenParen),            
            Token::new(TokenType::IntLit(101)),
            Token::new(TokenType::ClosedParen),            
        ]);
        assert_eq!(lexer::run(String::from("return(123)")).unwrap(), vec![
            Token::new(TokenType::Ret),
            Token::new(TokenType::OpenParen),            
            Token::new(TokenType::IntLit(123)),
            Token::new(TokenType::ClosedParen),            
        ]);    
    }
    #[test]
    fn lexer_return_multiple_number_negative() {
        assert_eq!(lexer::run(String::from("return(-91)")).unwrap(), vec![
            Token::new(TokenType::Ret),
            Token::new(TokenType::OpenParen),            
            Token::new(TokenType::IntLit(-91)),
            Token::new(TokenType::ClosedParen),            
        ]);
        assert_eq!(lexer::run(String::from("return(-101)")).unwrap(), vec![
            Token::new(TokenType::Ret),
            Token::new(TokenType::OpenParen),            
            Token::new(TokenType::IntLit(-101)),
            Token::new(TokenType::ClosedParen),            
        ]);
        assert_eq!(lexer::run(String::from("return(-123)")).unwrap(), vec![
            Token::new(TokenType::Ret),
            Token::new(TokenType::OpenParen),            
            Token::new(TokenType::IntLit(-123)),
            Token::new(TokenType::ClosedParen),            
        ]);  
    }
    #[test]
    fn lexer_empty_string() {
        assert_eq!(lexer::run(String::new()), None)
    }
    #[test]
    fn lexer_parentesis() {
        assert_eq!(lexer::run("((())".to_string()).unwrap(), vec![
            Token::new(TokenType::OpenParen),
            Token::new(TokenType::OpenParen),
            Token::new(TokenType::OpenParen),
            Token::new(TokenType::ClosedParen),
            Token::new(TokenType::ClosedParen),
        ]);
    }
    #[test]
    fn lexer_numbers() {
        assert_eq!(lexer::run("1213 sasasa -123".to_string()).unwrap(), vec![
            Token::new(TokenType::IntLit(1213)),
            Token::new(TokenType::Ident(String::from("sasasa"))),
            Token::new(TokenType::IntLit(-123)),
        ]); 
    }
    #[test]
    fn lexer_keywords() {
        assert_eq!(lexer::run("var x = 25 \n return(x)".to_string()).unwrap(), vec![
            Token::new(TokenType::Var),
            Token::new(TokenType::Ident(String::from("x"))),
            Token::new(TokenType::Eq),
            Token::new(TokenType::IntLit(25)),
            Token::new(TokenType::Ret), 
            Token::new(TokenType::OpenParen),
            Token::new(TokenType::Ident(String::from("x"))),
            Token::new(TokenType::ClosedParen),            
        ]); 
    }
}