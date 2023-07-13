use crate::{lexer::LexResult, token::Tok, Mode};
use itertools::{Itertools, MultiPeek};

/// An [`Iterator`] that transforms a token stream to accommodate soft keywords (namely, `match`
/// `case`, and `type`).
///
/// [PEP 634](https://www.python.org/dev/peps/pep-0634/) introduced the `match` and `case` keywords
/// as soft keywords, meaning that they can be used as identifiers (e.g., variable names) in certain
/// contexts.
///
/// Later, [PEP 695](https://peps.python.org/pep-0695/#generic-type-alias) introduced the `type`
/// soft keyword.
///
/// This function modifies a token stream to accommodate this change. In particular, it replaces
/// soft keyword tokens with `identifier` tokens if they are used as identifiers.
///
/// Handling soft keywords in this intermediary pass allows us to simplify both the lexer and
/// parser, as neither of them need to be aware of soft keywords.
pub struct SoftKeywordTransformer<I>
where
    I: Iterator<Item = LexResult>,
{
    underlying: MultiPeek<I>,
    start_of_line: bool,
}

impl<I> SoftKeywordTransformer<I>
where
    I: Iterator<Item = LexResult>,
{
    pub fn new(lexer: I, mode: Mode) -> Self {
        Self {
            underlying: lexer.multipeek(), // spell-checker:ignore multipeek
            start_of_line: matches!(mode, Mode::Interactive | Mode::Module),
        }
    }
}

impl<I> Iterator for SoftKeywordTransformer<I>
where
    I: Iterator<Item = LexResult>,
{
    type Item = LexResult;

    #[inline]
    fn next(&mut self) -> Option<LexResult> {
        let mut next = self.underlying.next();
        if let Some(Ok((tok, range))) = next.as_ref() {
            // If the token is a soft keyword e.g. `type`, `match`, or `case`, check if it's
            // used as an identifier. We assume every soft keyword use is an identifier unless
            // a heuristic is met.

            // For `match` and `case`, all of the following conditions must be met:
            // 1. The token is at the start of a logical line.
            // 2. The logical line contains a top-level colon (that is, a colon that is not nested
            //    inside a parenthesized expression, list, or dictionary).
            // 3. The top-level colon is not the immediate sibling of a soft keyword token.
            //    (This is to avoid treating soft keywords as identifiers when annotated with
            //    type hints.)
            if matches!(tok, Tok::Match | Tok::Case) {
                if !self.start_of_line {
                    next = Some(Ok((soft_to_name(tok), *range)));
                } else {
                    let mut nesting = 0;
                    let mut first = true;
                    let mut seen_colon = false;
                    let mut seen_lambda = false;
                    while let Some(Ok((tok, _))) = self.underlying.peek() {
                        match tok {
                            Tok::Newline => break,
                            Tok::Lambda if nesting == 0 => seen_lambda = true,
                            Tok::Colon if nesting == 0 => {
                                if seen_lambda {
                                    seen_lambda = false;
                                } else if !first {
                                    seen_colon = true;
                                }
                            }
                            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => nesting += 1,
                            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => nesting -= 1,
                            _ => {}
                        }
                        first = false;
                    }
                    if !seen_colon {
                        next = Some(Ok((soft_to_name(tok), *range)));
                    }
                }
            }
            // For `type` all of the following conditions must be met:
            // 1. The token is at the start of a logical line.
            // 2. The type token is followed by a name token.
            // 3. The name token is followed by an equality token.
            else if matches!(tok, Tok::Type) {
                if !self.start_of_line {
                    next = Some(Ok((soft_to_name(tok), *range)));
                } else {
                    let mut nesting = 0;
                    let mut seen_name = false;
                    let mut seen_equal = false;
                    while let Some(Ok((tok, _))) = self.underlying.peek() {
                        match tok {
                            Tok::Newline => break,
                            Tok::Name { .. } if nesting == 0 => seen_name = true,
                            Tok::Equal if nesting == 0 && seen_name => seen_equal = true,
                            Tok::Lpar | Tok::Lsqb | Tok::Lbrace => nesting += 1,
                            Tok::Rpar | Tok::Rsqb | Tok::Rbrace => nesting -= 1,
                            _ => {}
                        }
                    }
                    if !(seen_name && seen_equal) {
                        next = Some(Ok((soft_to_name(tok), *range)));
                    }
                }
            }
        }

        self.start_of_line = next.as_ref().map_or(false, |lex_result| {
            lex_result.as_ref().map_or(false, |(tok, _)| {
                #[cfg(feature = "full-lexer")]
                if matches!(tok, Tok::NonLogicalNewline | Tok::Comment { .. }) {
                    return self.start_of_line;
                }

                matches!(
                    tok,
                    Tok::StartModule
                        | Tok::StartInteractive
                        | Tok::Newline
                        | Tok::Indent
                        | Tok::Dedent
                )
            })
        });

        next
    }
}

#[inline]
fn soft_to_name(tok: &Tok) -> Tok {
    let name = match tok {
        Tok::Match => "match",
        Tok::Case => "case",
        Tok::Type => "type",
        _ => unreachable!("other tokens never reach here"),
    };
    Tok::Name {
        name: name.to_owned(),
    }
}
