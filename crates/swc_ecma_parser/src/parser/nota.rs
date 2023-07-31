use swc_atoms::Atom;
use swc_common::Span;
use swc_ecma_ast::{Expr, NotaTemplate};

use crate::{error::SyntaxError, token::*, PResult, Parser, Tokens};

#[allow(dead_code)]
impl<I: Tokens> Parser<I> {
    pub(super) fn parse_nota_template(&mut self) -> PResult<Box<Expr>> {
        let mut exprs = vec![];

        let start_pos: swc_common::BytePos = cur_pos!(self);

        if !eat!(self, "@{") {
            unexpected!(self, "Nota template start");
        }

        while let Some(elem) = self.parse_nota_template_elem().transpose() {
            exprs.push(elem?);
        }

        let expr = Box::new(Expr::NotaTemplate(NotaTemplate {
            span: span!(self, start_pos),
            exprs,
        }));

        Ok(expr)
    }

    pub fn parse_nota_template_elem(&mut self) -> PResult<Option<Atom>> {
        match cur!(self, true)? {
            Token::NotaText { raw } => {
                let output = raw.clone();
                bump!(self);
                Ok(Some(output))
            }
            tok!('}') => {
                bump!(self);
                Ok(None)
            }
            _ => unexpected!(self, "Nota template element"),
        }
    }
}
