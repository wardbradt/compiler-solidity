//!
//! The compiler lexer.
//!

pub mod error;
pub mod token;

#[cfg(test)]
mod tests;

use self::error::Error;
use self::token::lexeme::comment::Comment;
use self::token::lexeme::identifier::Identifier;
use self::token::lexeme::literal::integer::Integer as IntegerLiteral;
use self::token::lexeme::literal::string::String as StringLiteral;
use self::token::lexeme::symbol::Symbol;
use self::token::lexeme::Lexeme;
use self::token::location::Location;
use self::token::Token;

///
/// The compiler lexer.
///
pub struct Lexer {
    /// The input source code.
    input: String,
    /// The number of characters processed so far.
    offset: usize,
    /// The current location.
    location: Location,
    /// The peeked lexeme, waiting to be fetched.
    peeked: Option<Token>,
}

impl Lexer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(mut input: String) -> Self {
        input.push('\n');

        Self {
            input,
            offset: 0,
            location: Location::default(),
            peeked: None,
        }
    }

    ///
    /// Advances the lexer, returning the next lexeme.
    ///
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<Token, Error> {
        if let Some(peeked) = self.peeked.take() {
            return Ok(peeked);
        }

        while let Some(character) = self.input.chars().nth(self.offset) {
            if character.is_ascii_whitespace() {
                if character == '\n' {
                    self.location.line += 1;
                    self.location.column = 1;
                } else if character != '\r' {
                    self.location.column += 1;
                }
                self.offset += 1;
                continue;
            }

            if let Some(token) = Comment::parse(&self.input[self.offset..]) {
                self.offset += token.length;
                self.location
                    .shift_down(token.location.line, token.location.column);
                continue;
            }

            if let Some(mut token) = StringLiteral::parse(&self.input[self.offset..]) {
                token.location = self.location;

                self.offset += token.length;
                self.location.shift_right(token.length);
                return Ok(token);
            }

            if let Some(mut token) = IntegerLiteral::parse(&self.input[self.offset..]) {
                token.location = self.location;

                self.offset += token.length;
                self.location.shift_right(token.length);
                return Ok(token);
            }

            if let Some(mut token) = Identifier::parse(&self.input[self.offset..]) {
                token.location = self.location;

                self.offset += token.length;
                self.location.shift_right(token.length);
                return Ok(token);
            }

            if let Some(mut token) = Symbol::parse(&self.input[self.offset..]) {
                token.location = self.location;

                self.offset += token.length;
                self.location.shift_right(token.length);
                return Ok(token);
            }

            let end = self.input[self.offset..]
                .find(char::is_whitespace)
                .unwrap_or(self.input.len());
            return Err(Error::InvalidLexeme {
                location: self.location,
                sequence: self.input[self.offset..self.offset + end].to_owned(),
            });
        }

        Ok(Token::new(self.location, Lexeme::EndOfFile, 0))
    }

    ///
    /// Peeks the next lexeme without advancing the iterator.
    ///
    pub fn peek(&mut self) -> Result<Token, Error> {
        match self.peeked {
            Some(ref peeked) => Ok(peeked.clone()),
            None => {
                let peeked = self.next()?;
                self.peeked = Some(peeked.clone());
                Ok(peeked)
            }
        }
    }
}
