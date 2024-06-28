#[cfg(test)]
mod tests {
    use crate::{lexer::lexer::Token, lx, parser::parser::parse};

    #[test]
    fn lexer_return_multiple_number_positive() {
        assert_eq!(lx::run("return 91".to_string()).unwrap(), vec![
            Token::new(lx::TokenType::Ret, None),
            Token::new(lx::TokenType::IntLit, Some(91)),
            ]);

        assert_eq!(lx::run("return 101".to_string()).unwrap(), vec![
            Token::new(lx::TokenType::Ret, None),
            Token::new(lx::TokenType::IntLit, Some(101)),
        ]);
        assert_eq!(lx::run("return 123".to_string()).unwrap(), vec![
            Token::new(lx::TokenType::Ret, None),
            Token::new(lx::TokenType::IntLit, Some(123)),
            ]);    
    }
    #[test]
    fn lexer_return_multiple_number_negative() {
        assert_eq!(lx::run("return -91".to_string()).unwrap(), vec![
            Token::new(lx::TokenType::Ret, None),
            Token::new(lx::TokenType::IntLit, Some(-91)),

            ]);
        assert_eq!(lx::run("return -101".to_string()).unwrap(), vec![
                Token::new(lx::TokenType::Ret, None),
                Token::new(lx::TokenType::IntLit, Some(-101)),
        ]);
        assert_eq!(lx::run("return -123".to_string()).unwrap(), vec![
            Token::new(lx::TokenType::Ret, None),
            Token::new(lx::TokenType::IntLit, Some(-123)),
        ]); 
    }
    #[test]
    fn lexer_empty_string() {
        assert_eq!(lx::run(String::new()), None)
    }
    #[test]
    fn lexer_parentesis() {
        assert_eq!(lx::run("((())".to_string()).unwrap(), vec![
            Token::new(lx::TokenType::OpenParen, None),
            Token::new(lx::TokenType::OpenParen, None),
            Token::new(lx::TokenType::OpenParen, None),
            Token::new(lx::TokenType::ClosedParen, None),
            Token::new(lx::TokenType::ClosedParen, None),
        ]);
    }
    #[test]
    fn lexer_numbers() {
        assert_eq!(lx::run("1213 sasasa -123".to_string()).unwrap(), vec![
            Token::new(lx::TokenType::IntLit, Some(1213)),
            Token::new(lx::TokenType::IntLit, Some(-123)),
        ]); 
    }
    #[test]
    fn parser_empty_tokens() {
        assert_eq!(parse(vec![]), None);
    }
}