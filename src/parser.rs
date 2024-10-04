use crate::{lexer::tokenize, prelude::*};
use anyhow::{anyhow, Result};
use trees::Tree;

pub fn parse(tokens: &[Tokens], mut ast: Tree<Form>) -> Result<Tree<Form>> {
    let Some((head, tail)) = tokens.split_first() else {
        return Ok(ast);
    };

    match head {
        Tokens::Bounds(TokenBounds::LeftParen) => {
            let (Tokens::Symbol(sym), right_tokens) = tail
                .split_first()
                .ok_or(anyhow!("Unexpected empty parens"))?
            else {
                return Err(anyhow!(" "));
            };
            parse_into(
                Form::CallExpression(sym.clone()),
                OpenBoundsTracker::parens(),
                right_tokens,
                ast,
            )
        }
        Tokens::Bounds(TokenBounds::LeftBracket) => {
            parse_into(Form::List, OpenBoundsTracker::brackets(), tail, ast)
        }

        Tokens::Bounds(TokenBounds::LeftCurlyBraces) => {
            parse_into(Form::Record, OpenBoundsTracker::braces(), tail, ast)
        }

        Tokens::Bounds(_) => parse(tail, ast),

        token => {
            let form: Form = token.clone().into();
            ast.push_back(Tree::new(form));
            parse(tail, ast)
        }
    }
}

fn parse_into(
    form: Form,
    tracker: OpenBoundsTracker,
    tokens: &[Tokens],
    mut ast: Tree<Form>,
) -> Result<Tree<Form>> {
    let (inner, rest) = split_at_bound(tokens, tracker)?;
    ast.push_back(parse(inner, Tree::new(form))?);
    let parsed = parse(rest, Tree::new(Form::Root))?;
    parsed.iter().for_each(|n| ast.push_back(n.deep_clone()));
    Ok(ast)
}

fn split_at_bound(
    tokens: &[Tokens],
    mut tracker: OpenBoundsTracker,
) -> Result<(&[Tokens], &[Tokens])> {
    let split_index = tokens
        .iter()
        .position(|t| tracker.track(t))
        .ok_or(anyhow!("Uneven number of bound tokens"))?;
    Ok(tokens.split_at(split_index))
}

struct OpenBoundsTracker {
    opener: TokenBounds,
    closer: TokenBounds,
    count: i32,
}
impl OpenBoundsTracker {
    fn track(&mut self, bound: &Tokens) -> bool {
        self.count += match bound {
            Tokens::Bounds(b) if *b == self.opener => 1,
            Tokens::Bounds(b) if *b == self.closer => -1,
            _ => 0,
        };
        self.count == 0
    }

    fn parens() -> Self {
        OpenBoundsTracker {
            opener: TokenBounds::LeftParen,
            closer: TokenBounds::RightParen,
            count: 1,
        }
    }
    fn brackets() -> Self {
        OpenBoundsTracker {
            opener: TokenBounds::LeftBracket,
            closer: TokenBounds::RightBracket,
            count: 1,
        }
    }
    fn braces() -> Self {
        OpenBoundsTracker {
            opener: TokenBounds::LeftCurlyBraces,
            closer: TokenBounds::RightCurlyBraces,
            count: 1,
        }
    }
}

pub fn parse_string(code: &str) -> Result<Tree<Form>> {
    let ast = parse(&tokenize(code), Tree::new(Form::Root))?;
    Ok(ast)
}
