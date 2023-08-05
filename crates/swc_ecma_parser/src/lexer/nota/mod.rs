use smartstring::LazyCompact;
use swc_common::SyntaxContext;

pub use self::token::NotaToken;
use super::*;
use crate::Tokens;

mod token;

/// What kind of tokens to emit.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NotaTokenContext {
    /// Text and markup.
    Markup,
    /// Math atoms, operators, etc.
    Math,
    /// Keywords, literals and operators.
    Code,
}

impl Lexer<'_> {
    fn nota_context(&self) -> Option<NotaTokenContext> {
        match self.token_context().current() {
            Some(TokenContext::Nota(ctx)) => Some(ctx),
            _ => None,
        }
    }

    pub(super) fn maybe_read_nota_template(&mut self) -> LexResult<Option<Token>> {
        let c = self.cur();
        let start = self.cur_pos();

        if let Some(ctx) = self.nota_context() {
            let token = match self.cur() {
                Some(c) if c.is_whitespace() => self.nota_whitespace(),
                // Some('/') if self.s.eat(b'/') => self.line_comment(),
                // Some('/') if self.s.eat(b'*') => self.block_comment(),
                // Some('*') if self.s.eat(b'/') => self.error("unexpected end of block comment"),
                Some(c) => match ctx {
                    NotaTokenContext::Markup => self.nota_markup(c)?,
                    NotaTokenContext::Math => todo!(), // self.math(start, c),
                    NotaTokenContext::Code => todo!(), //self.code(start, c),
                },
                None => NotaToken::Eof,
            };
            println!("GOT EM: {token:?}");
            return Ok(Some(Token::Nota(token)));
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

    fn nota_whitespace(&mut self) -> NotaToken {
        let more = self.eat_while(char::is_whitespace);
        let newlines = count_newlines(more);

        if self.nota_context() == Some(NotaTokenContext::Markup) && newlines >= 2 {
            NotaToken::Parbreak
        } else {
            NotaToken::Space {
                newline: newlines > 0,
            }
        }
    }

    fn nota_markup(&mut self, c: char) -> LexResult<NotaToken> {
        self.bump();
        match c {
            '*' if !self.in_word() => Ok(NotaToken::Star),
            '-' if self.space_or_end() => Ok(NotaToken::ListMarker),
            '}' => Ok(NotaToken::RightBrace),
            _ => self.nota_text(c),
        }
    }

    fn nota_text(&mut self, c: char) -> LexResult<NotaToken> {
        macro_rules! table {
            ($(|$c:literal)*) => {
                static TABLE: [bool; 128] = {
                    let mut t = [false; 128];
                    $(t[$c as usize] = true;)*
                    t
                };
            };
        }

        table! {
            | ' ' | '\t' | '\n' | '\x0b' | '\x0c' | '\r' | '\\' | '/'
            | '[' | ']' | '{' | '}' | '~' | '-' | '.' | '\'' | '"'
            | '*' | '_' | ':' | 'h' | '`' | '$' | '<' | '>' | '@' | '#'
        };

        self.with_buf(|l, out| {
            out.push(c);
            loop {
                l.eat_until(|c: char| {
                    let stop = TABLE
                        .get(c as usize)
                        .copied()
                        .unwrap_or_else(|| c.is_whitespace());
                    if !stop {
                        out.push(c);
                    }
                    stop
                });

                // Continue with the same text node if the thing would become text
                // anyway.
                match l.cur() {
                    Some(' ') if l.at(char::is_alphanumeric) => {}
                    // Some('/') if !s.at(['/', '*']) => {}
                    // Some('-') if !s.at(['-', '?']) => {}
                    // Some('.') if !s.at("..") => {}
                    // Some('h') if !s.at("ttp://") && !s.at("ttps://") => {}
                    // Some('@') if !s.at(is_id_start) => {}
                    _ => break,
                };

                let c = l.cur().unwrap();
                l.bump();
                out.push(c);
            }

            Ok(NotaToken::Text {
                value: (&**out).into(),
            })
        })
    }

    #[inline]
    fn scout(&self, n: isize) -> Option<char> {
        let mut pos = self.cur_pos();
        if n > 0 {
            pos = pos + BytePos(n as u32);
        } else {
            pos = pos - BytePos((-n) as u32);
        }
        self.input.char_at(pos)
    }

    /// Tests if lexing in the middle of a word,
    /// i.e., there are alphanumeric chars on both sides of the current
    /// position.
    #[inline]
    fn in_word(&self) -> bool {
        let alphanum = |c: Option<char>| c.map_or(false, |c| c.is_alphanumeric());
        let prev = self.scout(-2);
        let cur = self.cur();
        alphanum(prev) && alphanum(cur)
    }

    /// Tests if the current character is the end of a word,
    /// i.e., the next char is either whitespace or EOF
    #[inline]
    fn space_or_end(&self) -> bool {
        match self.cur() {
            Some(c) => c.is_whitespace(),
            None => true,
        }
    }
}

#[inline]
pub fn is_newline(character: char) -> bool {
    matches!(
        character,
        // Line Feed, Vertical Tab, Form Feed, Carriage Return.
        '\n' | '\x0B' | '\x0C' | '\r' |
        // Next Line, Line Separator, Paragraph Separator.
        '\u{0085}' | '\u{2028}' | '\u{2029}'
    )
}

fn count_newlines(s: &str) -> usize {
    let mut newlines = 0;
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if is_newline(c) {
            if c == '\r' {
                it.next();
            }
            newlines += 1;
        }
    }
    newlines
}
