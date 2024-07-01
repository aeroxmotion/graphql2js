use crate::AstDescription;
use crate::AstDirectivesConst;
use crate::AstEnumValue;
use crate::AstLocation;
use crate::AstName;

pub mod _3_9_1_enum_extensions;

pub use _3_9_1_enum_extensions::*;

/// EnumTypeDefinition :
///   `Description`? enum `Name` `Directives[Const]`? `EnumValuesDefinition`
///   `Description`? enum `Name` `Directives[Const]`? [lookahead != {\]
///
/// Spec: https://spec.graphql.org/draft/#EnumTypeDefinition
#[derive(Clone, Debug)]
pub struct AstEnumTypeDefinition {
	/// `Description`?
	pub description: Option<AstDescription>,

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

/// EnumValuesDefinition :
///   { `EnumValueDefinition`+ }
///
/// Spec: https://spec.graphql.org/draft/#EnumValuesDefinition
#[derive(Clone, Debug)]
pub struct AstEnumValuesDefinition {
	/// `EnumValueDefinition`+
	pub definitions: Vec<AstEnumValueDefinition>,

	/// Node's location
	pub location: AstLocation,
}

/// EnumValueDefinition :
///   `Description`? `EnumValue` `Directives[Const]`?
///
/// Spec: https://spec.graphql.org/draft/#EnumValueDefinition
#[derive(Clone, Debug)]
pub struct AstEnumValueDefinition {
	/// `Description`?
	pub description: Option<AstDescription>,

	/// `EnumValue`
	pub value: AstEnumValue,

	/// `Directives[Const]`?
	pub directives: Option<AstDirectivesConst>,

	/// Node's location
	pub location: AstLocation,
}
