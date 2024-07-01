use graphql_common::location::AstLocation;

use crate::AstDirectivesConst;
use crate::AstEnumValuesDefinition;
use crate::AstName;

/// EnumTypeExtension :
///   extend enum `Name` `Directives[Const]`? `EnumValuesDefinition`
///   extend enum `Name` `Directives[Const]` [lookahead != {\]
///
/// Spec: https://spec.graphql.org/draft/#EnumTypeExtension
#[derive(Clone, Debug)]
pub struct AstEnumTypeExtension {
	/// `Name`
	pub name: AstName,

	/// `Directives[Const]`?
	pub directives: Option<AstDirectivesConst>,

	/// `EnumValuesDefinition`
	/// [lookahead != {\]
	pub definitions: Option<AstEnumValuesDefinition>,

	/// Node's location
	pub location: AstLocation,
}
