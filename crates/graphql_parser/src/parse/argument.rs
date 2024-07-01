use crate::{
	ast::{argument::AstArgument, location::AstLocation},
	error::ParserError,
	Parser,
};
use graphql_token::punctuator::{CLOSE_PARENTHESIS_PUNCTUATOR, COLON_PUNCTUATOR};

use super::{name::parse_name, value::parse_value};

/// Parses `Arguments` / `Argument[List]`
pub fn parse_arguments(parser: &mut Parser) -> Result<Vec<AstArgument>, ParserError> {
	// Skip `(`
	parser.next();

	let mut arguments = vec![];

	while !parser.is_punctuator(CLOSE_PARENTHESIS_PUNCTUATOR) {
		arguments.push(parse_argument(parser)?);
	}

	// Skip `)`
	parser.next();

	Ok(arguments)
}

/// Parses `Argument`
pub fn parse_argument(parser: &mut Parser) -> Result<AstArgument, ParserError> {
	let name = parse_name(parser)?;

	// Skip `:`
	parser.expect_punctuator(COLON_PUNCTUATOR)?;

	Ok(AstArgument {
		name,
		value: parse_value(parser)?,
		location: AstLocation {},
	})
}
