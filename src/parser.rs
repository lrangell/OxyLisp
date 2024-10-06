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
                return Err(anyhow!("Expected symbol after left paren"));
            };
            parse_into(
                Form::CallExpression(sym.clone()),
                TokenBounds::LeftParen,
                right_tokens,
                ast,
            )
        }
        Tokens::Bounds(TokenBounds::LeftBracket) => {
            parse_into(Form::List, TokenBounds::LeftBracket, tail, ast)
        }
        Tokens::Bounds(TokenBounds::LeftCurlyBraces) => {
            parse_into(Form::Record, TokenBounds::LeftCurlyBraces, tail, ast)
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
    opener: TokenBounds,
    tokens: &[Tokens],
    mut ast: Tree<Form>,
) -> Result<Tree<Form>> {
    let (inner, rest) = find_matching_bound(tokens, opener)?;
    ast.push_back(parse(inner, Tree::new(form))?);
    let parsed = parse(rest, Tree::new(Form::Root))?;
    parsed.iter().for_each(|n| ast.push_back(n.deep_clone()));
    Ok(ast)
}

fn find_matching_bound(tokens: &[Tokens], opener: TokenBounds) -> Result<(&[Tokens], &[Tokens])> {
    let closer = match opener {
        TokenBounds::LeftParen => TokenBounds::RightParen,
        TokenBounds::LeftBracket => TokenBounds::RightBracket,
        TokenBounds::LeftCurlyBraces => TokenBounds::RightCurlyBraces,
        _ => return Err(anyhow!("Invalid opener token")),
    };

    let mut count = 1;
    for (i, token) in tokens.iter().enumerate() {
        match token {
            Tokens::Bounds(b) if *b == opener => count += 1,
            Tokens::Bounds(b) if *b == closer => count -= 1,
            _ => {}
        }
        if count == 0 {
            return Ok(tokens.split_at(i + 1));
        }
    }
    Err(anyhow!("Uneven number of bound tokens"))
}

pub fn parse_string(code: &str) -> Result<Tree<Form>> {
    let ast = parse(&tokenize(code), Tree::new(Form::Root))?;
    Ok(ast)
}
