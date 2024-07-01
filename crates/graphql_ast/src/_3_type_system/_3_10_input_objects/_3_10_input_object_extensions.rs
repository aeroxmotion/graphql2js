use graphql_common::location::AstLocation;

use crate::AstDirectivesConst;
use crate::AstInputFieldsDefinition;
use crate::AstName;

/// InputObjectTypeExtension :
///   extend input `Name` `Directives[Const]`? `InputFieldsDefinition`
///   extend input `Name` `Directives[Const]` [lookahead != {\]
///
/// Spec: https://spec.graphql.org/draft/#InputObjectTypeExtension
#[derive(Clone, Debug)]
pub struct AstInputObjectTypeExtension {
	/// `Name`
	pub name: AstName,

	/// `Directives[Const]`?
	pub directives: Option<AstDirectivesConst>,

	/// `InputFieldsDefinition`
	/// [lookahead != {\]
	pub definitions: Option<AstInputFieldsDefinition>,

	/// Node's location
	pub location: AstLocation,
}
