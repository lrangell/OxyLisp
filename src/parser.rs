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
                // let (_, args) = rest.split_last().expect("4c");
                let n = rest
                    .iter()
                    .rposition(|t| match t {
                        Tokens::Bounds(TokenBounds::RightParen) => true,
                        _ => false,
                    })
                    .unwrap();
                let (args, rr) = rest.split_at(n);
                let mut ca = [Form::CallExpression((sym.clone(), parse(args).unwrap()))].to_vec();
                let rrr = parse(rr).unwrap();
                ca.extend(rrr);
                return Ok(ca);
            } else {
                return Err(anyhow!("First element of a form must be a symbol"));
            }
        }
        Tokens::Literal(l) => {
            let uu = parse(tail)?;
            let mut r = [Form::Literal(l.to_owned())].to_vec();
            r.extend(uu);
            Ok(r)
        }
        Tokens::Bounds(TokenBounds::RightParen) => parse(tail),
        Tokens::Symbol(s) => {
            let uu = parse(tail)?;
            let mut r = [Form::Symbol(s.to_owned())].to_vec();
            r.extend(uu);
            Ok(r)
        } // &s => Ok(vec![s.into()]),
    }
}
