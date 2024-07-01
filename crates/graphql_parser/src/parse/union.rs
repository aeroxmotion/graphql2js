use super::{directive::parse_directives, name::parse_name};
use crate::{
	ast::{
		location::AstLocation,
		type_::{reference::AstNamedType, union::AstUnionTypeDefinition},
		value::string::AstStringValue,
	},
	error::ParserError,
	Parser,
};
use graphql_token::punctuator::{AMPERSAND_PUNCTUATOR, EQUALS_PUNCTUATOR};

/// Parses `UnionTypeDefinition`
pub fn parse_union_type_definition(
	parser: &mut Parser,
	description: Option<AstStringValue>,
) -> Result<AstUnionTypeDefinition, ParserError> {
	// Skip `union` keyword
	parser.next();

	Ok(AstUnionTypeDefinition {
		description,
		name: parse_name(parser)?,
		directives: parse_directives(parser)?,
		members: parse_union_member_types(parser)?,
		location: AstLocation {},
	})
}

/// Parses `UnionMemberTypes`
pub fn parse_union_member_types(parser: &mut Parser) -> Result<Vec<AstNamedType>, ParserError> {
	let mut members = vec![];

	if !parser.skip_if_punctuator(EQUALS_PUNCTUATOR) {
		return Err(ParserError::UnexpectedToken(parser.token().clone()));
	}

	// Try to skip `&`
	parser.skip_if_punctuator(AMPERSAND_PUNCTUATOR);

	loop {
		members.push(parse_name(parser)?);

		// Try to skip `|`
		if !parser.skip_if_punctuator(AMPERSAND_PUNCTUATOR) {
			// Break if not skipped
			break;
		}
	}

	Ok(members)
}
