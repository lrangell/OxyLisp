use std::borrow::Borrow;

use crate::lexer::*;
use crate::parser::*;
use crate::types::*;

pub struct CodeChunk {
    pub tokens: Vec<Tokens>,
}

impl CodeChunk {
    pub fn new(code: &str) -> Self {
        Self {
            tokens: tokenize(code),
        }
    }
}

impl fmt::Display for CodeChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.tokens)
    }
}

// const closing_pair: HashMap<TokenBounds, TokenBounds> =
//     HashMap::from([(TokenBounds::OpeningParen, TokenBounds::ClosingParen)]);

fn is_matching_pair(opening: &TokenBounds, closing: &Tokens) -> bool {
    // debug!("Finding if {} is a closing pair of {}", closing, opening);
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
        // debug!("CodeChunk Iterator: {:?}", self.tokens);
        let mut token_iter = self.tokens.iter();
        let next_token = token_iter.next()?;
        // debug!("Parsing token: {}", next_token);

        let form = match next_token {
            Tokens::TokenBounds(opening) => {
                token_iter.advance_back_by(1).ok()?;
                // let tt = token_iter
                //     .by_ref()
                //     .take_while(|closing| !is_matching_pair(&opening, *closing))
                //     .cloned()
                //     .collect();
                let tt = token_iter.by_ref().cloned().collect();
                debug!("Tokens between (): {:?}", tt);
                ast(tt)
            }
            Tokens::Primitive(p) => Some(p.clone().into()),
            _ => None,
        };
        let remaing_tokens = token_iter.cloned().collect();
        debug!("Remaining tokens: {:?}", remaing_tokens);

        self.tokens = remaing_tokens;
        form
    }
}
