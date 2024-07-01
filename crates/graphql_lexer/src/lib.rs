#![feature(assert_matches)]
pub mod error;
pub(crate) mod ignored;
pub(crate) mod name;
pub(crate) mod number;
pub mod punctuator;
pub(crate) mod static_semantics;
pub(crate) mod string;

use graphql_token::location::Location;
use graphql_token::punctuator::*;
use graphql_token::token::Token;

use error::LexerError;
use ignored::*;
use name::*;
use number::*;
use punctuator::*;
use string::*;

pub(crate) type TokenResult = Result<Token, LexerError>;
pub(crate) type LexerResult = Result<Vec<Token>, LexerError>;

macro_rules! match_punctuator {
	($lexer:ident, $pattern:pat) => {
		!$lexer.end() && matches!($lexer.peek(), $pattern)
	};
}

pub(crate) use match_punctuator;

pub struct Lexer {
	/// The source code
	pub source: String,

	/// Points to current scanned index's source
	pub(crate) cursor: usize,

	/// Current line scanned
	pub(crate) line: usize,

	/// Current column scanned
	pub(crate) column: usize,

	/// Source's characters (Unicode scalar values)
	pub(crate) chars: Vec<char>,

	/// Source's characters length
	pub(crate) len: usize,
}

impl Lexer {
	pub fn new<T: Into<String>>(source: T) -> Self {
		let _source: String = source.into();
		let chars: Vec<char> = _source.chars().collect();

		Self {
			len: chars.len(),
			chars,

			source: _source,
			cursor: 0,
			line: 1,
			column: 1,
		}
	}

	pub fn tokenize<T: Into<String>>(src: T) -> LexerResult {
		let mut lexer = Self::new(src);

		lexer.scan()
	}
}

impl Lexer {
	pub(crate) fn end(&self) -> bool {
		self.cursor >= self.len
	}

	pub(crate) fn ends_by(&self, delta: usize) -> bool {
		self.cursor + delta >= self.len
	}

	pub(crate) fn try_peek(&self) -> Result<char, LexerError> {
		if self.end() {
			Err(LexerError::UnexpectedEOF)
		} else {
			Ok(self.peek())
		}
	}

	pub(crate) fn peek(&self) -> char {
		self.peek_by(0)
	}

	pub(crate) fn peek_by(&self, delta: usize) -> char {
		self.chars[self.cursor + delta]
	}

	pub(crate) fn try_is(&self, char: char) -> Result<bool, LexerError> {
		if self.end() {
			Err(LexerError::UnexpectedEOF)
		} else {
			Ok(self.peek() == char)
		}
	}

	pub(crate) fn is(&self, char: char) -> bool {
		!self.end() && self.peek() == char
	}

	pub(crate) fn is_by(&self, char: char, delta: usize) -> bool {
		!self.ends_by(delta) && self.peek_by(delta) == char
	}

	pub(crate) fn lookahead(&self, char: char, len: usize) -> bool {
		if self.ends_by(len) {
			false
		} else {
			for delta in 1..=len {
				if !self.is_by(char, delta) {
					return false;
				}
			}

			true
		}
	}

	pub(crate) fn eat(&mut self) {
		self.eat_by(1);
	}

	pub(crate) fn eat_by(&mut self, amount: usize) {
		self.column += amount;
		self.cursor += amount;
	}

	pub(crate) fn eat_line(&mut self) {
		self.line += 1;
		self.cursor += 1;
		self.column = 1;
	}

	/// Returns current's lexer location
	pub(crate) fn loc(&self) -> Location {
		Location {
			index: self.cursor,
			line: self.line,
			column: self.column,
		}
	}

	pub fn scan(&mut self) -> LexerResult {
		let mut tokens = vec![Token::SOF];

		'root: while !self.end() {
			let token = match self.peek() {
				// Ignored ::
				//
				// Spec: https://spec.graphql.org/draft/#sec-Language.Source-Text.Ignored-Tokens

				//   UnicodeBOM
				//   WhiteSpace
				//   Comma
				UNICODE_BOM | white_space!() | COMMA => {
					self.eat();
					continue;
				}

				//   LineTerminator :: Carriage Return  New Line
				CARRIAGE_RETURN if self.is_by(NEW_LINE, 1) => {
					self.eat();
					self.eat_line();
					continue;
				}

				//   LineTerminator ::
				//     New Line
				//     Carriage Return [lookahead != New Line]
				line_terminator!() => {
					self.eat_line();
					continue;
				}

				//   Comment ::
				//     # `CommentChar` *list*,*opt* [lookahead != `CommentChar`]
				//
				//   CommentChar ::
				// 		   `SourceCharacter` but not `LineTerminator`
				NUMBER_SIGN => {
					loop {
						// Skip `CommentChar`
						self.eat();

						// Skip until encounter `LF` / `CR` / <EOF>
						if self.end() || matches!(self.peek(), line_terminator!()) {
							continue 'root;
						}
					}
				}

				// Token::
				//
				// Spec: https://spec.graphql.org/draft/#sec-Language.Source-Text.Lexical-Tokens

				//   Punctuator
				//   ...
				punctuator!() => try_parse_punctuator(self),
				PERIOD_PUNCTUATOR if self.lookahead(PERIOD_PUNCTUATOR, 2) => {
					try_parse_triple_period_punctuator(self)
				}

				//   ...
				//   Name
				//   ...
				name_start_pattern!() => try_parse_name(self),

				//   ...
				//   IntValue
				//   FloatValue
				//   ...
				NEGATIVE_SIGN => try_parse_signed_number(self),
				digit!() => try_parse_unsigned_number(self),

				//   ...
				//   StringValue
				DOUBLE_QUOTE => {
					if self.lookahead(DOUBLE_QUOTE, 2) {
						try_parse_block_string(self)
					} else {
						try_parse_string_value(self)
					}
				}

				// --- Unknown tokens ---
				char => Err(LexerError::UnknownToken(char, self.loc())),
			}?;

			tokens.push(token);
		}

		tokens.push(Token::EOF);
		Ok(tokens)
	}
}

#[cfg(test)]
mod tests {
	use super::Lexer;

	macro_rules! assert_loc {
		// <LINE>:<COLUMN>:<CURSOR>
		($lexer:ident, $line:literal:$column:literal:$cursor:literal) => {
			assert_eq!($lexer.line, $line);
			assert_eq!($lexer.column, $column);
			assert_eq!($lexer.cursor, $cursor);
		};
	}

	#[test]
	fn it_should_be_eating_correctly() {
		let mut lexer = Lexer::new("");

		// Initial state
		//   Line: 1
		//   Column: 2
		//   Cursor: 0
		assert_loc!(lexer, 1:1:0);

		// Advance by `1`
		//   Line: 1
		//   Column: 2
		//   Cursor: 1
		lexer.eat();
		assert_loc!(lexer, 1:2:1);

		// Advance by `3`
		//   Line: 1
		//   Column: 5
		//   Cursor: 4
		lexer.eat_by(3);
		assert_loc!(lexer, 1:5:4);
	}

	#[test]
	fn it_should_be_peeking_correctly() {
		let mut lexer = Lexer::new("12345");

		assert_eq!(lexer.peek(), '1');

		lexer.eat();

		assert_eq!(lexer.peek(), '2');
		assert_eq!(lexer.peek_by(3), '5');
	}
}
