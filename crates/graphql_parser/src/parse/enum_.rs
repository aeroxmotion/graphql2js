use super::{description::parse_description, directive::parse_directives, name::parse_name};
use crate::{
	ast::{
		location::AstLocation,
		type_::{
			enum_::{AstEnumTypeDefinition, AstEnumValueDefinition},
			reference::AstName,
		},
		value::string::AstStringValue,
	},
	error::ParserError,
	Parser,
};
use graphql_token::punctuator::{CLOSING_BRACE_PUNCTUATOR, OPENING_BRACE_PUNCTUATOR};

/// Parses `EnumTypeDefinition`
pub fn parse_enum_type_definition(
	parser: &mut Parser,
	description: Option<AstStringValue>,
) -> Result<AstEnumTypeDefinition, ParserError> {
	// Skip `enum` keyword
	parser.next();

	Ok(AstEnumTypeDefinition {
		description,
		name: parse_name(parser)?,
		directives: parse_directives(parser)?,
		values: parse_enum_values_definition(parser)?,
		location: AstLocation {},
	})
}

/// Parses `EnumValuesDefinition`
pub fn parse_enum_values_definition(
	parser: &mut Parser,
) -> Result<Vec<AstEnumValueDefinition>, ParserError> {
	let mut values = vec![];

	// Try to skip `{`
	if parser.skip_if_punctuator(OPENING_BRACE_PUNCTUATOR) {
		while !parser.is_punctuator(CLOSING_BRACE_PUNCTUATOR) {
			values.push(parse_enum_value_definition(parser)?);
		}
	}

	Ok(values)
}

/// Parses `EnumValueDefinition`
pub fn parse_enum_value_definition(
	parser: &mut Parser,
) -> Result<AstEnumValueDefinition, ParserError> {
	let description = parse_description(parser);
	let value = parse_name(parser)?;

	let AstName {
		ref name,
		location: _,
	} = value;

	if matches!(name.as_str(), "true" | "false" | "null") {
		return Err(ParserError::UnexpectedToken(parser.prev_token().clone()));
	}

	Ok(AstEnumValueDefinition {
		description,
		value,
		directives: parse_directives(parser)?,
		location: AstLocation {},
	})
}
