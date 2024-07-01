use crate::location::TokenLocation;

///
pub enum GraphQLError {
	///
	Lexer(GraphQLLexerError),

	///
	Parser(GraphQLParserError),
}

///
pub enum GraphQLLexerError {
	/// Unknown token error
	UnknownToken(char, TokenLocation),

	/// Unexpected token error
	InvalidToken(char, TokenLocation),

	/// Invalid unicode sequence
	InvalidUnicode(TokenLocation),

	/// Unexpected End-Of-File
	UnexpectedEOF,
}

///
pub enum GraphQLParserError {}
