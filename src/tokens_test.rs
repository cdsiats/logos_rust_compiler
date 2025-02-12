#[cfg(test)]
mod tests {
    use logos::Logos;
    use crate::tokens::Token;
    use super::*;

    #[test]
    fn should_tokenize_identifier() {
        let input = "camelCaseIdentifier PascalCaseIdentifier UPPERCASE lowercase";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lexer.slice(), "camelCaseIdentifier");
        
        assert_eq!(lexer.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lexer.slice(), "PascalCaseIdentifier");

        assert_eq!(lexer.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lexer.slice(), "UPPERCASE");

        assert_eq!(lexer.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lexer.slice(), "lowercase");
    }

    #[test]
    fn should_tokenize_literals() {
        let input = "\"hello\" 44 -44 44.4 -44.4";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Literal)));
        assert_eq!(lexer.slice(), "\"hello\"");

        assert_eq!(lexer.next(), Some(Ok(Token::Literal)));
        assert_eq!(lexer.slice(), "44");

        assert_eq!(lexer.next(), Some(Ok(Token::Literal)));
        assert_eq!(lexer.slice(), "-44");

        assert_eq!(lexer.next(), Some(Ok(Token::Literal)));
        assert_eq!(lexer.slice(), "44.4");

        assert_eq!(lexer.next(), Some(Ok(Token::Literal)));
        assert_eq!(lexer.slice(), "-44.4");
    }

    #[test]
    fn should_tokenize_booleans() {
        let input = "true false";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::Literal)));
        assert_eq!(lexer.slice(), true.to_string());

        assert_eq!(lexer.next(), Some(Ok(Token::Literal)));
        assert_eq!(lexer.slice(), false.to_string());
    }

    #[test]
    fn tokenize_symbols() {
        let input = "( ) { } [ ]";
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(Token::OpenParen)));
        assert_eq!(lexer.slice(), "(");

        assert_eq!(lexer.next(), Some(Ok(Token::CloseParen)));
        assert_eq!(lexer.slice(), ")");

        assert_eq!(lexer.next(), Some(Ok(Token::OpenBrace)));
        assert_eq!(lexer.slice(), "{");

        assert_eq!(lexer.next(), Some(Ok(Token::CloseBrace)));
        assert_eq!(lexer.slice(), "}");

        assert_eq!(lexer.next(), Some(Ok(Token::OpenSquare)));
        assert_eq!(lexer.slice(), "[");

        assert_eq!(lexer.next(), Some(Ok(Token::CloseSquare)));
        assert_eq!(lexer.slice(), "]");
    }
}