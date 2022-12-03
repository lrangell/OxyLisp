use std::borrow::Borrow;

use crate::lexer::*;
use crate::parser::*;
use crate::types::*;

pub struct CodeChunk {
    tokens: Vec<Tokens>,
}

impl CodeChunk {
    fn new(code: &str) -> Self {
        Self {
            tokens: tokenize(code),
        }
    }
}

// const closing_pair: HashMap<TokenBounds, TokenBounds> =
//     HashMap::from([(TokenBounds::OpeningParen, TokenBounds::ClosingParen)]);

fn is_matching_pair(opening: &TokenBounds, closing: &Tokens) -> bool {
    let closing_pair: HashMap<TokenBounds, TokenBounds> =
        HashMap::from([(TokenBounds::OpeningParen, TokenBounds::ClosingParen)]);
    if let Some(pair) = closing_pair.get(opening) {
        return match closing {
            Tokens::TokenBounds(a) => *a == *pair,
            _ => false,
        };
    }
    false
}

impl Iterator for CodeChunk {
    type Item = Form;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token_iter = self.tokens.iter();
        let next_token = token_iter.next()?;

        let form = match next_token {
            Tokens::TokenBounds(opening) => ast(token_iter
                .by_ref()
                .take_while(|closing| is_matching_pair(&opening, closing))
                .cloned()
                .collect()),
            Tokens::Primitive(p) => Some(p.clone().into()),
            _ => None,
        };

        self.tokens = token_iter.cloned().collect();
        form
    }
}
