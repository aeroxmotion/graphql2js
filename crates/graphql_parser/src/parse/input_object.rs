use super::{directive::parse_directives, name::parse_name, object::parse_input_value_definition};
use crate::{
	ast::{
		location::AstLocation,
		type_::{input_object::AstInputObjectTypeDefinition, object::AstInputValueDefinition},
		value::string::AstStringValue,
	},
	error::ParserError,
	Parser,
};
use graphql_token::punctuator::{CLOSING_BRACE_PUNCTUATOR, OPENING_BRACE_PUNCTUATOR};

/// Parses `InputObjectTypeDefinition`
pub fn parse_input_object_type_definition(
	parser: &mut Parser,
	description: Option<AstStringValue>,
) -> Result<AstInputObjectTypeDefinition, ParserError> {
	// Skip `input` keyword
	parser.next();

	Ok(AstInputObjectTypeDefinition {
		description,
		name: parse_name(parser)?,
		directives: parse_directives(parser)?,
		fields: parse_input_fields_definition(parser)?,
		location: AstLocation {},
	})
}

/// Parses `InputFieldsDefinition`
pub fn parse_input_fields_definition(
	parser: &mut Parser,
) -> Result<Vec<AstInputValueDefinition>, ParserError> {
	let mut fields = vec![];

	if parser.skip_if_punctuator(OPENING_BRACE_PUNCTUATOR) {
		while !parser.is_punctuator(CLOSING_BRACE_PUNCTUATOR) {
			fields.push(parse_input_value_definition(parser)?);
		}
	}

	Ok(fields)
}
