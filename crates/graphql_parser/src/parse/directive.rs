use graphql_token::{
	punctuator::{AT_SIGN_PUNCTUATOR, VERTICAL_BAR_PUNCTUATOR},
	token::Token,
};

use crate::{
	ast::{
		directive::AstDirective,
		location::AstLocation,
		type_::{
			directive::{AstDirectiveDefinition, AstTypeSystemDirectiveLocation},
			reference::AstName,
		},
		value::string::AstStringValue,
	},
	error::ParserError,
	Parser,
};

use super::{argument::parse_arguments, name::parse_name, object::parse_arguments_definition};

/// Parses `DirectiveDefinition`
pub fn parse_directive_definition(
	parser: &mut Parser,
	description: Option<AstStringValue>,
) -> Result<AstDirectiveDefinition, ParserError> {
	// Skip `directive` keyword
	parser.next();

	parser.expect_punctuator(AT_SIGN_PUNCTUATOR)?;

	Ok(AstDirectiveDefinition {
		description,
		name: parse_name(parser)?,
		arguments: parse_arguments_definition(parser)?,
		repeatable: parse_repeatable_keyword(parser),
		locations: parse_directive_locations(parser)?,
	})
}

/// Parses `repeatable` keyword
pub fn parse_repeatable_keyword(parser: &mut Parser) -> Option<AstName> {
	let repeatable = match parser.token() {
		Token::Name(name, _) => Some(AstName {
			name: name.clone(),
			location: AstLocation {},
		}),
		_ => None,
	};

	if repeatable.is_some() {
		// Skip `repeatable` keyword
		parser.next();
	}

	repeatable
}

/// Parses `DirectiveLocations`
pub fn parse_directive_locations(
	parser: &mut Parser,
) -> Result<Vec<AstTypeSystemDirectiveLocation>, ParserError> {
	// Skip `on` keyword
	parser.expect_name("on")?;

	// Try to skip `|`
	parser.skip_if_punctuator(VERTICAL_BAR_PUNCTUATOR); // TODO: Needed?

	let mut locations = vec![];

	loop {
		locations.push(parse_directive_location(parser)?);

		if !parser.skip_if_punctuator(VERTICAL_BAR_PUNCTUATOR) {
			break;
		}
	}

	Ok(locations)
}

pub fn parse_directive_location(
	parser: &mut Parser,
) -> Result<AstTypeSystemDirectiveLocation, ParserError> {
	let location = match parser.token() {
		Token::Name(value, _) => {
			let name = AstName {
				name: value.clone(),
				location: AstLocation {},
			};

			match value.as_str() {
				"SCHEMA" => Ok(AstTypeSystemDirectiveLocation::Schema(name)),
				"SCALAR" => Ok(AstTypeSystemDirectiveLocation::Scalar(name)),
				"OBJECT" => Ok(AstTypeSystemDirectiveLocation::Object(name)),
				"FIELD_DEFINITION" => Ok(AstTypeSystemDirectiveLocation::FieldDefinition(name)),
				"ARGUMENT_DEFINITION" => {
					Ok(AstTypeSystemDirectiveLocation::ArgumentDefinition(name))
				}
				"INTERFACE" => Ok(AstTypeSystemDirectiveLocation::Interface(name)),
				"UNION" => Ok(AstTypeSystemDirectiveLocation::Union(name)),
				"ENUM" => Ok(AstTypeSystemDirectiveLocation::Enum(name)),
				"ENUM_VALUE" => Ok(AstTypeSystemDirectiveLocation::EnumValue(name)),
				"INPUT_OBJECT" => Ok(AstTypeSystemDirectiveLocation::InputObject(name)),
				"INPUT_FIELD_DEFINITION" => {
					Ok(AstTypeSystemDirectiveLocation::InputFieldDefinition(name))
				}
				_ => Err(ParserError::UnexpectedToken(parser.token().clone())),
			}
		}
		token => Err(ParserError::UnexpectedToken(token.clone())),
	};

	if location.is_ok() {
		// Skip `name`
		parser.next();
	}

	location
}

/// Parses `Directive[List]`
pub fn parse_directives(parser: &mut Parser) -> Result<Vec<AstDirective>, ParserError> {
	let mut directives = vec![];

	while matches!(parser.token(), Token::Punctuator(punctuator, _) if punctuator == &AT_SIGN_PUNCTUATOR)
	{
		directives.push(parse_directive(parser)?);
	}

	Ok(directives)
}

/// Parses `Directive`
pub fn parse_directive(parser: &mut Parser) -> Result<AstDirective, ParserError> {
	// Skip `@`
	parser.next();

	Ok(AstDirective {
		name: parse_name(parser)?,
		arguments: parse_arguments(parser)?,
		location: AstLocation {},
	})
}
