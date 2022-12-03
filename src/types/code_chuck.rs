pub struct CodeChunk<'a> {
    tokens: Vec<Tokens>,
    token_iter: Box<dyn Iterator<Item = &'a Tokens>>,
}

pub impl CodeChunk<'_> {
    fn new(code: String) -> Self {
        let tokens = tokenize(code);
        let tokenIter = Box::new(tokens.iter());
        Self {
            tokens,
            token_iter: tokenIter,
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

pub impl Iterator for CodeChunk<'_> {
    type Item = Form;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.token_iter.next() {
            return match *token {
                Tokens::TokenBounds(opening) => ast(self
                    .token_iter
                    .take_while(|&closing| is_matching_pair(&opening, closing))
                    .cloned()
                    .collect()),
                Tokens::Primitive(p) => Some(p.into()),
                _ => None,
            };
        }
        None
    }
}
