use crate::{
	ast::{location::AstLocation, value::string::AstStringValue},
	Parser,
};
use graphql_token::token::Token;

/// Parses `Description`
pub fn parse_description(parser: &mut Parser) -> Option<AstStringValue> {
	let value = match parser.token() {
		Token::String(value, _) => Some(AstStringValue {
			value: value.clone(),
			location: AstLocation {},
			block: false,
		}),
		Token::BlockString(value, _) => Some(AstStringValue {
			value: value.clone(),
			location: AstLocation {},
			block: true,
		}),
		_ => None,
	}?;

	// Skip `Description`
	parser.next();

	Some(value)
}
