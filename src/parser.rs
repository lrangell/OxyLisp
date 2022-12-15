use crate::types::*;
use anyhow::{anyhow, Result};
use std::vec::Vec;

pub fn parse(tokens: &[Tokens]) -> Result<Vec<Form>> {
    if tokens.is_empty() {
        let r: Vec<Form> = [].to_vec();
        return Ok(r);
    }
    let (head, tail) = tokens
        .split_first()
        .ok_or(anyhow!("\nEmpty token vector\n"))?;

    debug!("head: {:?} tail: {:?}", head, tail);

    match &head {
        Tokens::Bounds(TokenBounds::LeftParen) => {
            if let (Tokens::Symbol(sym), right_tokens) =
                tail.split_first().ok_or("Unexpected empty parens")?
            {
                let (args_tokens, rest) =
                    split_at_bound(right_tokens, Tokens::Bounds(TokenBounds::RightParen))?;
                let args = parse(args_tokens)?;
                let remaining_forms = parse(rest)?;
                let mut call_expr = vec![Form::CallExpression((sym.clone(), args))];
                call_expr.extend(remaining_forms);
                return Ok(call_expr);
            } else {
                return Err(anyhow!("First element of a form must be a symbol"));
            }
        }
        Tokens::Bounds(TokenBounds::LeftBracket) => {
            let (inner, rest) = split_at_bound(tail, Tokens::Bounds(TokenBounds::RightBracket))?;
            let inner_forms = parse(inner)?;
            let rest_forms = parse(rest)?;
            let mut list_form = vec![Form::List(Box::new(inner_forms))];
            list_form.extend(rest_forms);
            Ok(list_form)
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
            let remaining_forms = parse(tail)?;
            let mut symbol_form = [Form::Symbol(s.to_owned())].to_vec();
            symbol_form.extend(remaining_forms);
            Ok(symbol_form)
        } // &s => Ok(vec![s.into()]),
    }
}

fn split_at_bound(tokens: &[Tokens], closing_bound: Tokens) -> Result<(&[Tokens], &[Tokens])> {
    let n = tokens
        .iter()
        .rposition(|t| *t == closing_bound)
        .ok_or(anyhow!(
            "\n\nUnable to find closing delimiter: {}\nIn the tokens: {:?}\n\n",
            closing_bound,
            tokens
        ))?;
    let (inner, rest) = tokens.split_at(n);
    Ok((inner, rest))
}
