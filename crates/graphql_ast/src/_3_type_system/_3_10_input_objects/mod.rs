use crate::AstDescription;
use crate::AstDirectivesConst;
use crate::AstInputValueDefinition;
use crate::AstLocation;
use crate::AstName;

pub mod _3_10_input_object_extensions;

pub use _3_10_input_object_extensions::*;

/// InputObjectTypeDefinition :
///   `Description`? input `Name` `Directives[Const]`? `InputFieldsDefinition`
///   `Description`? input `Name` `Directives[Const]`? [lookahead != {\]
///
/// Spec: https://spec.graphql.org/draft/#InputObjectTypeDefinition
#[derive(Clone, Debug)]
pub struct AstInputObjectTypeDefinition {
	/// `Description`?
	pub description: Option<AstDescription>,

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

/// InputFieldsDefinition :
///   { `InputValueDefinition`+ }
///
/// Spec: https://spec.graphql.org/draft/#InputFieldsDefinition
#[derive(Clone, Debug)]
pub struct AstInputFieldsDefinition {
	/// `InputValueDefinition`+
	pub definitions: Vec<AstInputValueDefinition>,

	/// Node's location
	pub location: AstLocation,
}
