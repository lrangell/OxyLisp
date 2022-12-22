use crate::types::*;
use anyhow::{anyhow, Result};
use log::debug;
use std::vec::Vec;

pub fn parse(tokens: &[Tokens]) -> Result<Vec<Form>> {
    if tokens.is_empty() {
        let r: Vec<Form> = [].to_vec();
        return Ok(r);
    }
    let (head, tail) = tokens
        .split_first()
        .ok_or(anyhow!("\nEmpty token vector\n"))?;
    // debug!("\nhead: {:?}\ntail: {:?}", head, tail);

    match &head {
        Tokens::Bounds(TokenBounds::LeftParen) => {
            if let (Tokens::Symbol(sym), right_tokens) = tail
                .split_first()
                .ok_or(anyhow!("Unexpected empty parens"))?
            {
                let (inner, rest) =
                    split_at_bound(right_tokens, OpenBoundsTracker::paren_tracker())?;
                let call_expr = Form::CallExpression((sym.clone(), parse(inner)?));
                return parse_remaing(call_expr, rest);
            } else {
                return Err(anyhow!("First element of a form must be a symbol"));
            }
        }
        Tokens::Bounds(TokenBounds::LeftBracket) => {
            let (inner, rest) = split_at_bound(tail, OpenBoundsTracker::bracket_tracker())?;
            let list = Form::List(Box::new(parse(inner)?));
            parse_remaing(list, rest)
        }
        Tokens::Literal(l) => parse_remaing(l.to_owned().into(), tail),
        Tokens::Bounds(TokenBounds::RightParen) => parse(tail),
        Tokens::Bounds(TokenBounds::RightBracket) => parse(tail),
        Tokens::Symbol(s) => parse_remaing(Form::Symbol(s.to_owned()), tail),
    }
}

fn parse_remaing(form: Form, rest: &[Tokens]) -> Result<Vec<Form>> {
    Ok(vec![form].into_iter().chain(parse(rest)?).collect())
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

    fn paren_tracker() -> Self {
        OpenBoundsTracker {
            opener: TokenBounds::LeftParen,
            closer: TokenBounds::RightParen,
            count: 1,
        }
    }
    fn bracket_tracker() -> Self {
        OpenBoundsTracker {
            opener: TokenBounds::LeftBracket,
            closer: TokenBounds::RightBracket,
            count: 1,
        }
    }
}
