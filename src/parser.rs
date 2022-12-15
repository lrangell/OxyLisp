use crate::types::*;
use anyhow::{anyhow, Result};
use std::vec::Vec;

pub fn parse(tokens: &[Tokens]) -> Result<Vec<Form>> {
    if tokens.is_empty() {
        let r: Vec<Form> = [].to_vec();
        return Ok(r);
    }
    let (head, tail) = tokens.split_first().unwrap();

    debug!("head: {:?} tail: {:?}", head, tail);

    match &head {
        Tokens::Bounds(TokenBounds::LeftParen) => {
            if let (Tokens::Symbol(sym), rest) = tail.split_first().unwrap() {
                let (args, rr) = split_at_bound(rest, Tokens::Bounds(TokenBounds::RightParen))?;
                let mut ca = [Form::CallExpression((sym.clone(), parse(args).unwrap()))].to_vec();
                let rrr = parse(rr).unwrap();
                ca.extend(rrr);
                return Ok(ca);
            } else {
                return Err(anyhow!("First element of a form must be a symbol"));
            }
        }
        Tokens::Bounds(TokenBounds::LeftBracket) => {
            let (inner, rest) = split_at_bound(tokens, Tokens::Bounds(TokenBounds::RightBracket))?;
            let inner_forms = parse(inner)?;
            let rest_forms = parse(rest)?;
            let mut a = vec![Form::List(Box::new(inner_forms))];
            a.extend(rest_forms);
            Ok(a)
        }
        Tokens::Literal(l) => {
            let uu = parse(tail)?;
            let mut r = [Form::Literal(l.to_owned())].to_vec();
            r.extend(uu);
            Ok(r)
        }
        Tokens::Bounds(TokenBounds::RightParen) => parse(tail),
        Tokens::Bounds(TokenBounds::RightBracket) => parse(tail),
        Tokens::Symbol(s) => {
            let uu = parse(tail)?;
            let mut r = [Form::Symbol(s.to_owned())].to_vec();
            r.extend(uu);
            Ok(r)
        } // &s => Ok(vec![s.into()]),
    }
}

fn split_at_bound(tokens: &[Tokens], closing_bound: Tokens) -> Result<(&[Tokens], &[Tokens])> {
    let n = tokens
        .iter()
        .rposition(|t| *t == closing_bound)
        .ok_or(anyhow!(""))?;
    let (inner, rest) = tokens.split_at(n);
    Ok((inner, rest))
}
