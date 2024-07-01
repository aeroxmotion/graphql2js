use graphql_token::token::Token;

use crate::{
	ast::{location::AstLocation, type_::reference::AstName},
	error::ParserError,
	Parser,
};

/// Parses `Name`
pub fn parse_name(parser: &mut Parser) -> Result<AstName, ParserError> {
	match parse_name_optional(parser) {
		None => Err(ParserError::UnexpectedToken(parser.token().clone())),
		Some(name) => Ok(name),
	}
}

/// Parses `Name[Opt]`
pub fn parse_name_optional(parser: &mut Parser) -> Option<AstName> {
	let name = match parser.token() {
		Token::Name(name, _) => Some(AstName {
			name: name.clone(),
			location: AstLocation {},
		}),
		_ => None,
	}?;

	// Skip `name`
	parser.next();

	Some(name)
}
