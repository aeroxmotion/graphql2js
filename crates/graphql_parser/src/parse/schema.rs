use super::{directive::parse_directives, name::parse_name};
use crate::{
	ast::{
		location::AstLocation,
		type_::schema::{AstOperationType, AstRootOperationTypeDefinition, AstSchemaDefinition},
		value::string::AstStringValue,
	},
	error::ParserError,
	Parser,
};
use graphql_token::{
	punctuator::{CLOSING_BRACE_PUNCTUATOR, COLON_PUNCTUATOR, OPENING_BRACE_PUNCTUATOR},
	token::Token,
};

/// Parses `SchemaDefinition`
pub fn parse_schema_definition(
	parser: &mut Parser,
	description: Option<AstStringValue>,
) -> Result<AstSchemaDefinition, ParserError> {
	// Skip `schema`
	parser.next();

	let directives = parse_directives(parser)?;

	// Skip `{`
	parser.expect_punctuator(OPENING_BRACE_PUNCTUATOR)?;

	Ok(AstSchemaDefinition {
		description,
		directives,
		definitions: parse_root_operation_type_definitions(parser)?,
		location: AstLocation {},
	})
}

/// Parses `RootOperationTypeDefinition[List]`
pub fn parse_root_operation_type_definitions(
	parser: &mut Parser,
) -> Result<Vec<AstRootOperationTypeDefinition>, ParserError> {
	let mut definitions = vec![];

	while !parser.is_punctuator(CLOSING_BRACE_PUNCTUATOR) {
		definitions.push(parse_root_operation_type_definition(parser)?);
	}

	// Skip `}`
	parser.next();

	Ok(definitions)
}

/// Parses `RootOperationTypeDefinition`
pub fn parse_root_operation_type_definition(
	parser: &mut Parser,
) -> Result<AstRootOperationTypeDefinition, ParserError> {
	let token = parser.token();
	let operation = match token {
		Token::Name(name, _) => match name.as_str() {
			"query" => Ok(AstOperationType::Query(AstLocation {})),
			"mutation" => Ok(AstOperationType::Mutation(AstLocation {})),
			"subscription" => Ok(AstOperationType::Subscription(AstLocation {})),
			_ => Err(ParserError::UnexpectedToken(token.clone())),
		},
		_ => Err(ParserError::UnexpectedToken(token.clone())),
	}?;

	// Skip `:`
	parser.expect_punctuator(COLON_PUNCTUATOR)?;

	Ok(AstRootOperationTypeDefinition {
		operation,
		type_: parse_name(parser)?,
		location: AstLocation {},
	})
}
