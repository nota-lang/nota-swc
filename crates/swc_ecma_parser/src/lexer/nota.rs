use smartstring::LazyCompact;

use super::*;
use crate::Tokens;

impl Lexer<'_> {
    pub(super) fn maybe_read_nota_template(&mut self) -> LexResult<Option<Token>> {
        let c = self.cur();

        if self.token_context().current() == Some(TokenContext::NotaTemplate) {
            if c == Some('}') {
                self.bump();
                return Ok(Some(tok!('}')));
            }

            return self.read_nota_token().map(Some);
        }

        if let Some(c) = c {
            if c == '@' && self.input.peek() == Some('{') && self.state.is_expr_allowed {
                self.bump();
                self.bump();
                return Ok(Some(tok!("@{")));
            }
        }

        Ok(None)
    }

    pub(super) fn read_nota_token(&mut self) -> LexResult<Token> {
        let start = self.cur_pos();
        let mut raw = SmartString::<LazyCompact>::new();
        while let Some(c) = self.cur() {
            if c == '}' {
                return Ok(Token::NotaText {
                    raw: Atom::new(&*raw),
                });
            }

            self.bump();
            raw.push(c);
        }

        self.error(start, SyntaxError::UnterminatedTpl)
    }
}
