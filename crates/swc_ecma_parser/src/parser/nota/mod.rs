use swc_atoms::Atom;
use swc_common::{BytePos, Span};
use swc_ecma_ast::{
    Expr, NotaElem, NotaElems, NotaListItem, NotaParbreak, NotaSpace, NotaStrong, NotaTemplate,
    NotaText,
};

use crate::{
    error::SyntaxError,
    lexer::{nota::is_newline, NotaToken},
    token::*,
    PResult, Parser, Tokens,
};

pub type FileId = ();

macro_rules! nota_eat {
    ($p:expr, $t:pat) => {{
        if matches!($p.input.cur(), Some(Token::Nota($t))) {
            bump!($p);
            true
        } else {
            false
        }
    }};
}

macro_rules! nota_cur {
    ($p:expr, $b:expr) => {{
        let Token::Nota(tok) = cur!($p, $b)? else {
            unexpected!($p, "Not a Nota token")
        };
        tok
    }};
}

macro_rules! nota_bump {
    ($p:expr) => {{
        let Token::Nota(tok) = bump!($p) else {
            unexpected!($p, "Not a Nota token")
        };
        tok
    }};
}

#[allow(dead_code)]
impl<I: Tokens> Parser<I> {
    fn parse_nota_markup_elem(&mut self, at_start: &mut bool) -> PResult<NotaElem> {
        let start = cur_pos!(self);
        let output = match nota_bump!(self) {
            NotaToken::Space { .. } => NotaElem::Space(NotaSpace {
                span: span!(self, start),
            }),
            NotaToken::Parbreak => NotaElem::Parbreak(NotaParbreak {
                span: span!(self, start),
            }),
            NotaToken::Text { value } => NotaElem::Text(NotaText {
                span: span!(self, start),
                value,
            }),
            NotaToken::Star => NotaElem::Strong(self.parse_nota_strong(start)?),
            NotaToken::ListMarker if *at_start => NotaElem::List(self.parse_nota_list(start)?),
            _ => unexpected!(self, "Nota markup element"),
        };

        *at_start = false;
        Ok(output)
    }

    fn parse_nota_strong(&mut self, start: BytePos) -> PResult<NotaStrong> {
        let elems = self.parse_nota_markup(false, 0, |p| {
            Ok(match nota_cur!(p, false) {
                NotaToken::Star => {
                    bump!(p);
                    true
                }
                NotaToken::Parbreak => true,
                _ => false,
            })
        })?;
        Ok(NotaStrong {
            span: span!(self, start),
            elems,
        })
    }

    fn column(&self, at: BytePos) -> usize {
        self.input_ref().input()[..(at.0 as usize)]
            .chars()
            .rev()
            .take_while(|&c| !is_newline(c))
            .count()
    }

    fn parse_nota_list(&mut self, start: BytePos) -> PResult<NotaListItem> {
        let at = self.input.cur_pos();
        let min_indent = self.column(at) + 1;
        let elems = self.parse_nota_markup(false, min_indent, |p| Ok(false))?;
        Ok(NotaListItem {
            span: span!(self, start),
            elems,
        })
    }

    fn parse_nota_markup(
        &mut self,
        mut at_start: bool,
        min_indent: usize,
        mut stop: impl FnMut(&mut Parser<I>) -> PResult<bool>,
    ) -> PResult<NotaElems> {
        let mut elems = Vec::new();

        loop {
            if stop(self)? {
                break;
            }

            if matches!(nota_cur!(self, false), NotaToken::RightBrace) {
                break;
            }

            if self.newline()? {
                at_start = true;
                if min_indent > 0 {
                    break;
                }
                bump!(self);
                continue;
            }

            elems.push(self.parse_nota_markup_elem(&mut at_start)?);
        }

        Ok(elems)
    }

    fn newline(&mut self) -> PResult<bool> {
        Ok(match nota_cur!(self, false) {
            NotaToken::Parbreak => true,
            NotaToken::Space { newline } => *newline,
            _ => false,
        })
    }

    pub(super) fn parse_nota_template(&mut self) -> PResult<Box<Expr>> {
        let start_pos: swc_common::BytePos = cur_pos!(self);

        if !eat!(self, "@{") {
            unexpected!(self, "Nota template start");
        }

        let elems = self.parse_nota_markup(true, 0, |p| Ok(false))?;
        bump!(self);

        let expr = Box::new(Expr::NotaTemplate(NotaTemplate {
            span: span!(self, start_pos),
            elems,
        }));

        Ok(expr)
    }
}
