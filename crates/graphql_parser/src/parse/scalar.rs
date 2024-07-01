use super::{directive::parse_directives, name::parse_name};
use crate::{
	ast::{
		location::AstLocation, type_::scalar::AstScalarTypeDefinition,
		value::string::AstStringValue,
	},
	error::ParserError,
	Parser,
};

/// Parses `ScalarTypeDefinition`
pub fn parse_scalar_type_definition(
	parser: &mut Parser,
	description: Option<AstStringValue>,
) -> Result<AstScalarTypeDefinition, ParserError> {
	// Skip `scalar`
	parser.next();

	Ok(AstScalarTypeDefinition {
		description,
		name: parse_name(parser)?,
		directives: parse_directives(parser)?,
		location: AstLocation {},
	})
}
