pub mod error;
pub mod parse;
pub mod visitor;

use graphql_ast::kind::document::AstTypeSystemDocument;
use graphql_token::token::Token;

use error::ParserError;
use parse::document::parse_type_system_document;

const SKIP_SOF_TOKEN: usize = 1;

pub struct Parser {
	/// Previously scanned tokens
	pub tokens: Vec<Token>,

	/// Current token being processed
	pub cursor: usize,
}

impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self {
			tokens,
			cursor: SKIP_SOF_TOKEN,
		}
	}
}

impl Parser {
	pub fn token(&self) -> &Token {
		&self.tokens[self.cursor]
	}

	pub fn next(&mut self) {
		self.cursor += 1;
	}

	pub(crate) fn prev_token(&self) -> &Token {
		&self.tokens[self.cursor - 1]
	}

	/// Skips whether current parser's `token` equals to given `punctuator`
	/// Returns `true` if skipped, `false` otherwise.
	pub fn skip_if_punctuator(&mut self, punctuator: char) -> bool {
		if matches!(self.token(), Token::Punctuator(value, _) if value == &punctuator) {
			self.next();
			true
		} else {
			false
		}
	}

	/// Skips whether current parser's `token` equals to given `name`
	/// Returns `true` if skipped, `false` otherwise.
	pub fn skip_if_name(&mut self, name: &str) -> bool {
		if matches!(self.token(), Token::Name(value, _) if value == name) {
			self.next();
			true
		} else {
			false
		}
	}

	pub fn expect_punctuator(&mut self, punctuator: char) -> Result<u8, ParserError> {
		if matches!(self.token(), Token::Punctuator(value, _) if value == &punctuator) {
			self.next();
			Ok(1)
		} else {
			Err(ParserError::UnexpectedToken(self.token().clone()))
		}
	}

	pub fn expect_name(&mut self, name: &str) -> Result<u8, ParserError> {
		if matches!(self.token(), Token::Name(value, _) if value.as_str() == name) {
			self.next();
			Ok(1)
		} else {
			Err(ParserError::UnexpectedToken(self.token().clone()))
		}
	}

	pub fn is_punctuator(&self, punctuator: char) -> bool {
		matches!(self.token(), Token::Punctuator(value, _) if value == &punctuator)
	}

	pub fn parse(&mut self) -> Result<AstTypeSystemDocument, ParserError> {
		parse_type_system_document(self)
	}
}
