use crate::{lexer::tokenize, prelude::*};
use anyhow::{anyhow, Result};
use log::debug;
use std::{borrow::BorrowMut, vec::Vec};
use trees::{tr, Node, Tree};

pub fn parse(tokens: &[Tokens], ast: &mut Node<Form>) -> Result<()> {
    if tokens.is_empty() {
        return Ok(());
    }
    let (head, tail) = tokens
        .split_first()
        .ok_or(anyhow!("\nEmpty token vector\n"))?;

    match head {
        Tokens::Bounds(TokenBounds::LeftParen) => {
            let (Tokens::Symbol(sym), right_tokens) = tail
                .split_first()
                .ok_or(anyhow!("Unexpected empty parens"))? else {return Err(anyhow!(" "))};
            let (inner, rest) = split_at_bound(right_tokens, OpenBoundsTracker::paren_tracker())?;

            let mut args = Tree::new(Form::CallExpression(sym.clone()));

            parse(inner, &mut args.root_mut())?;
            ast.push_back(args);

            let mut dummy = Tree::new(Form::Root);
            parse(rest, &mut dummy.root_mut())?;
            ast.append(dummy.abandon());
        }
        Tokens::Bounds(TokenBounds::LeftBracket) => {
            let (inner, rest) = split_at_bound(tail, OpenBoundsTracker::bracket_tracker())?;

            let mut list = Tree::new(Form::List);

            parse(inner, &mut list.root_mut())?;
            ast.push_back(list);

            let mut dummy = Tree::new(Form::Root);
            parse(rest, &mut dummy.root_mut())?;
            ast.append(dummy.abandon());
        }
        Tokens::Bounds(_) => parse(tail, ast)?,

        token => {
            let form: Form = token.clone().into();
            ast.push_back(Tree::new(form));
            parse(tail, ast)?;
        }
    };
    Ok(())
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

pub fn parse_string(code: &str) -> Result<Tree<Form>> {
    let mut tree = Tree::new(Form::Root);
    let root = tree.root_mut().get_mut();
    parse(&tokenize(code), root)?;
    Ok(tree)
}
