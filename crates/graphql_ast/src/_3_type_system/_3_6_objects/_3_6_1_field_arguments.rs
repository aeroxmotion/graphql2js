use graphql_common::location::AstLocation;

use crate::AstDefaultValue;
use crate::AstDescription;
use crate::AstDirectives;
use crate::AstName;

/// ArgumentsDefinition :
///   ( `InputValueDefinition`+ )
///
/// Spec: https://spec.graphql.org/draft/#ArgumentsDefinition
#[derive(Clone, Debug)]
pub struct AstArgumentsDefinition {
	/// `InputValueDefinition`+
	pub definitions: Vec<AstInputValueDefinition>,

	/// Node's location
	pub location: AstLocation,
}

/// InputValueDefinition :
///   `Description`? `Name` : `Type` `DefaultValue`? `Directives[Const]`?
///
/// Spec: https://spec.graphql.org/draft/#InputValueDefinition
#[derive(Clone, Debug)]
pub struct AstInputValueDefinition {
	/// `Description`?
	pub description: Option<AstDescription>,

	/// `Name`
	pub name: AstName,

	/// `DefaultValue`?
	pub default_value: Option<AstDefaultValue>,

	/// `Directives[Const]`?
	pub directives: Option<AstDirectives>,

	/// Node's location
	pub location: AstLocation,
}
