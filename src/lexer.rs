use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Tokens {
    Invalid,
    OpeningParen,
    ClosingParen,
    Integer(i32),
    Symbol(String),
}

fn tokenize(expression: String) -> Vec<Tokens> {
    let isSymbol = Regex::new(r"[A-Za-z]").unwrap();
    let isInteger = Regex::new(r"\d+").unwrap();

    return expression
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|t| -> Tokens {
            match t {
                "(" => Tokens::OpeningParen,
                ")" => Tokens::ClosingParen,
                i if isSymbol.is_match(i) => Tokens::Symbol(i.to_string()),
                i if isInteger.is_match(i) => Tokens::Integer(i.parse::<i32>().unwrap()),
                i => Tokens::Symbol(i.to_string()),
            }
        })
        .collect::<Vec<_>>();
}

mod tests {
    use super::{tokenize, Tokens};

    #[test]
    fn test_tokenizer() {
        assert_eq!(
            vec![
                Tokens::OpeningParen,
                Tokens::Symbol("+".to_string()),
                Tokens::Integer(4),
                Tokens::Integer(2),
                Tokens::ClosingParen
            ],
            tokenize("(+ 4 2)".to_string())
        )
    }
}
