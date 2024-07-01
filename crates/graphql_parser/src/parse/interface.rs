use super::{directive::parse_directives, name::parse_name, object::parse_implements_interfaces};
use crate::{
	ast::{
		location::AstLocation, type_::interface::AstInterfaceTypeDefinition,
		value::string::AstStringValue,
	},
	error::ParserError,
	Parser,
};

/// Parses `InterfaceTypeDefinition`
pub fn parse_interface_type_definition(
	parser: &mut Parser,
	description: Option<AstStringValue>,
) -> Result<AstInterfaceTypeDefinition, ParserError> {
	// Skip `interface` keyword
	parser.next();

	Ok(AstInterfaceTypeDefinition {
		description,
		name: parse_name(parser)?,
		implements: parse_implements_interfaces(parser)?,
		directives: parse_directives(parser)?,
		location: AstLocation {},
	})
}
