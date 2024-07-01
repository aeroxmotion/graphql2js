use super::{
	description::parse_description, directive::parse_directives, name::parse_name,
	type_::parse_type, variable::parse_default_value,
};
use crate::{
	ast::{
		location::AstLocation,
		type_::{
			object::{AstFieldDefinition, AstInputValueDefinition, AstObjectTypeDefinition},
			reference::AstNamedType,
		},
		value::string::AstStringValue,
	},
	error::ParserError,
	Parser,
};
use graphql_token::punctuator::{
	AMPERSAND_PUNCTUATOR, CLOSING_BRACE_PUNCTUATOR, OPENING_BRACE_PUNCTUATOR,
};

/// Parses `ObjectTypeDefinition`
pub fn parse_object_type_definition(
	parser: &mut Parser,
	description: Option<AstStringValue>,
) -> Result<AstObjectTypeDefinition, ParserError> {
	// Skip `type`
	parser.next();

	Ok(AstObjectTypeDefinition {
		description,
		name: parse_name(parser)?,
		implements: parse_implements_interfaces(parser)?,
		directives: parse_directives(parser)?,
		fields: parse_field_definitions(parser)?,
		location: AstLocation {},
	})
}

/// Parses `ImplementsInterfaces`
pub fn parse_implements_interfaces(parser: &mut Parser) -> Result<Vec<AstNamedType>, ParserError> {
	let mut interfaces = vec![];

	// Try to skip `implements`
	if parser.skip_if_name("implements") {
		// Try to skip `&`
		parser.skip_if_punctuator(AMPERSAND_PUNCTUATOR);

		loop {
			interfaces.push(parse_name(parser)?);

			// Try to skip `&`
			if !parser.skip_if_punctuator(AMPERSAND_PUNCTUATOR) {
				break;
			}
		}
	}

	Ok(interfaces)
}

/// Parses `FieldDefinitions`
pub fn parse_field_definitions(
	parser: &mut Parser,
) -> Result<Vec<AstFieldDefinition>, ParserError> {
	let mut fields = vec![];

	// Try to skip `{`
	if parser.skip_if_punctuator(OPENING_BRACE_PUNCTUATOR) {
		while !parser.is_punctuator(CLOSING_BRACE_PUNCTUATOR) {
			fields.push(parse_field_definition(parser)?);
		}

		// Skip `}`
		parser.next();
	}

	Ok(fields)
}

/// Parses `FieldDefinition`
pub fn parse_field_definition(parser: &mut Parser) -> Result<AstFieldDefinition, ParserError> {
	Ok(AstFieldDefinition {
		description: parse_description(parser),
		name: parse_name(parser)?,
		arguments: parse_arguments_definition(parser)?,
		location: AstLocation {},
	})
}

/// Parses `ArgumentsDefinition`
pub fn parse_arguments_definition(
	parser: &mut Parser,
) -> Result<Vec<AstInputValueDefinition>, ParserError> {
	let mut arguments = vec![];

	if parser.skip_if_punctuator(OPENING_BRACE_PUNCTUATOR) {
		while !parser.is_punctuator(CLOSING_BRACE_PUNCTUATOR) {
			arguments.push(parse_input_value_definition(parser)?);
		}

		// Skip `)`
		parser.next();
	}

	Ok(arguments)
}

/// Parses `InputValueDefintion`
pub fn parse_input_value_definition(
	parser: &mut Parser,
) -> Result<AstInputValueDefinition, ParserError> {
	Ok(AstInputValueDefinition {
		description: parse_description(parser),
		name: parse_name(parser)?,
		type_: parse_type(parser)?,
		default_value: parse_default_value(parser)?,
		directives: parse_directives(parser)?,
		location: AstLocation {},
	})
}
