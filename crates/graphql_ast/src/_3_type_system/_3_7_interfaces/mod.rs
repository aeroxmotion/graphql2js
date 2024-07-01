use crate::AstDescription;
use crate::AstDirectivesConst;
use crate::AstFieldsDefinition;
use crate::AstImplementsInterfaces;
use crate::AstLocation;
use crate::AstName;

pub mod _3_7_1_interface_extensions;

pub use _3_7_1_interface_extensions::*;

/// InterfaceTypeDefinition :
///   `Description`? interface `Name` `ImplementsInterfaces`? `Directives[Const]`? `FieldsDefinition`
///   `Description`? interface `Name` `ImplementsInterfaces`? `Directives[Const]`? [lookahead != {\]
///
/// Spec: https://spec.graphql.org/draft/#InterfaceTypeDefinition
#[derive(Clone, Debug)]
pub struct AstInterfaceTypeDefinition {
	/// `Description`?
	pub description: Option<AstDescription>,

	/// `Name`
	pub name: AstName,

	/// `ImplementsInterface`>
	pub implements: Option<AstImplementsInterfaces>,

	/// `Directives[Const]`?
	pub directives: Option<AstDirectivesConst>,

	/// `FieldsDefinition`
	/// [lookahead != {\]
	pub fields: Option<AstFieldsDefinition>,

	/// Node's location
	pub location: AstLocation,
}
