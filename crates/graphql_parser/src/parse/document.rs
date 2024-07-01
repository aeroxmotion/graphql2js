use graphql_token::token::Token;

use crate::{
	ast::{
		location::AstLocation,
		type_::document::{AstTypeSystemDefinition, AstTypeSystemDocument},
	},
	error::ParserError,
	Parser,
};

use super::{
	description::parse_description, directive::parse_directive_definition,
	enum_::parse_enum_type_definition, input_object::parse_input_object_type_definition,
	interface::parse_interface_type_definition, object::parse_object_type_definition,
	scalar::parse_scalar_type_definition, schema::parse_schema_definition,
	union::parse_union_type_definition,
};

/// Parses `TypeSystemDocument`
pub fn parse_type_system_document(
	parser: &mut Parser,
) -> Result<AstTypeSystemDocument, ParserError> {
	let mut definitions = vec![];

	while !matches!(parser.token(), Token::EOF) {
		definitions.push(parse_type_system_definition(parser)?);
	}

	Ok(AstTypeSystemDocument {
		definitions,
		location: AstLocation {},
	})
}

/// Parses `TypeSystemDefinition`
pub fn parse_type_system_definition(
	parser: &mut Parser,
) -> Result<AstTypeSystemDefinition, ParserError> {
	let description = parse_description(parser);

	match parser.token() {
		token @ Token::Name(name, _) => match name.as_str() {
			"schema" => Ok(AstTypeSystemDefinition::Schema(parse_schema_definition(
				parser,
				description,
			)?)),
			"scalar" => Ok(AstTypeSystemDefinition::Scalar(
				parse_scalar_type_definition(parser, description)?,
			)),
			"type" => Ok(AstTypeSystemDefinition::Object(
				parse_object_type_definition(parser, description)?,
			)),
			"interface" => Ok(AstTypeSystemDefinition::Interface(
				parse_interface_type_definition(parser, description)?,
			)),
			"union" => Ok(AstTypeSystemDefinition::Union(parse_union_type_definition(
				parser,
				description,
			)?)),
			"enum" => Ok(AstTypeSystemDefinition::Enum(parse_enum_type_definition(
				parser,
				description,
			)?)),
			"input" => Ok(AstTypeSystemDefinition::InputObject(
				parse_input_object_type_definition(parser, description)?,
			)),
			"directive" => Ok(AstTypeSystemDefinition::Directive(
				parse_directive_definition(parser, description)?,
			)),
			_ => Err(ParserError::UnexpectedToken(token.clone())),
		},
		token => Err(ParserError::UnexpectedToken(token.clone())),
	}
}
