use graphql_common::error::GraphQLError;
use graphql_common::error::GraphQLLexerError;
use graphql_common::location::TokenLocation;
use graphql_common::result::GraphQLResult;

/// Lexer's state
pub struct LexerState {
	/// Source code
	pub source: String,

	/// Current's location
	location: TokenLocation,

	/// Unicode scalar values
	chars: Vec<char>,

	/// Total unicode scalar values
	len: usize,
}

impl LexerState {
	pub fn new<T: Into<String>>(src: T) -> Self {
		let source: String = src.into();
		let chars: Vec<char> = source.chars().collect();

		Self {
			source,
			location: TokenLocation::new(0, 1, 1),

			len: chars.len(),
			chars,
		}
	}
}

impl LexerState {
	pub fn index(&self) -> usize {
		self.location.index
	}

	pub fn end(&self) -> bool {
		self.ends_by(0)
	}

	pub fn ends_by(&self, delta: usize) -> bool {
		self.index() + delta >= self.len
	}

	pub fn peek(&self) -> char {
		self.peek_by(0)
	}

	pub fn peek_by(&self, delta: usize) -> char {
		self.chars[self.index() + delta]
	}

	pub fn try_peek(&self) -> GraphQLResult<char> {
		if self.end() {
			Err(GraphQLError::Lexer(GraphQLLexerError::UnexpectedEOF))
		} else {
			Ok(self.peek())
		}
	}

	pub fn is(&self, char: char) -> bool {
		self.is_by(char, 0)
	}

	pub fn is_by(&self, char: char, delta: usize) -> bool {
		!self.ends_by(delta) && self.peek_by(delta) == char
	}

	pub fn try_is(&self, char: char) -> GraphQLResult<bool> {
		if self.end() {
			Err(GraphQLError::Lexer(GraphQLLexerError::UnexpectedEOF))
		} else {
			Ok(self.peek() == char)
		}
	}

	pub fn lookahead(&self, char: char, len: usize) -> bool {
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

	pub fn loc(&self) -> TokenLocation {
		self.location.clone()
	}

	pub fn eat(&mut self) {
		self.eat_by(1);
	}

	pub fn eat_by(&mut self, amount: usize) {
		self.location.index += amount;
		self.location.column += amount;
	}

	pub fn eat_line(&mut self) {
		self.location.index += 1;
		self.location.line += 1;
		self.location.column = 1;
	}
}

#[cfg(test)]
mod tests {
	use super::LexerState;

	macro_rules! assert_loc {
		// <LINE>:<COLUMN>:<CURSOR>
		($state:ident, $line:literal:$column:literal:$index:literal) => {{
			let location = $state.loc();

			assert_eq!(location.line, $line);
			assert_eq!(location.column, $column);
			assert_eq!(location.index, $index);
		}};
	}

	#[test]
	fn it_should_be_eating_correctly() {
		let mut state = LexerState::new("");

		// Initial state
		//   Line: 1
		//   Column: 2
		//   Cursor: 0
		assert_loc!(state, 1:1:0);

		// Advance by `1`
		//   Line: 1
		//   Column: 2
		//   Cursor: 1
		state.eat();
		assert_loc!(state, 1:2:1);

		// Advance by `3`
		//   Line: 1
		//   Column: 5
		//   Cursor: 4
		state.eat_by(3);
		assert_loc!(state, 1:5:4);
	}

	#[test]
	fn it_should_be_peeking_correctly() {
		let mut state = LexerState::new("12345");

		assert_eq!(state.peek(), '1');

		state.eat();

		assert_eq!(state.peek(), '2');
		assert_eq!(state.peek_by(3), '5');
	}
}
