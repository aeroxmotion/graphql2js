use graphql_token::location::Location;

#[derive(Debug)]
pub enum LexerError {
	/// Unknown token error
	UnknownToken(char, Location),

	/// Unexpected token error
	InvalidToken(char, Location),

	/// Unexpected End-Of-File
	UnexpectedEOF,

	/// Invalid unicode sequence
	InvalidUnicode(Location),
}
