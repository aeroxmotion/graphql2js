use graphql_common::location::AstLocation;

use crate::AstDirectivesConst;
use crate::AstFieldsDefinition;
use crate::AstImplementsInterfaces;
use crate::AstName;

/// InterfaceTypeExtension :
///   extend interface `Name` `ImplementsInterfaces`? `Directives[Const]`? `FieldsDefinition`
///   extend interface `Name` `ImplementsInterfaces`? `Directives[Const]` [lookahead != {\]
///   extend interface `Name` `ImplementsInterfaces` [lookahead != {\]
///
/// Spec: https://spec.graphql.org/draft/#InterfaceTypeExtension
#[derive(Clone, Debug)]
pub struct AstInterfaceTypeExtension {
	/// `Name`
	pub name: AstName,

	/// `ImplementsInterfaces`?
	pub implements: Option<AstImplementsInterfaces>,

	/// `Directives[Const]`
	pub directives: Option<AstDirectivesConst>,

	/// `FieldsDefinition`
	/// [lookahead != {\]
	pub fields: Option<AstFieldsDefinition>,

	/// Node's location
	pub location: AstLocation,
}
