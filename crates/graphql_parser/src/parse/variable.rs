use crate::{ast::value::AstValue, error::ParserError, Parser};
use graphql_token::punctuator::EQUALS_PUNCTUATOR;

use super::value::parse_value;

/// Parses `DefaultValue`
pub fn parse_default_value(parser: &mut Parser) -> Result<Option<AstValue>, ParserError> {
	if parser.skip_if_punctuator(EQUALS_PUNCTUATOR) {
		Ok(Some(parse_value(parser)?))
	} else {
		Ok(None)
	}
}
