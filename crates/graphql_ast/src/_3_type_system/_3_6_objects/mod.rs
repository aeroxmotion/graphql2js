use crate::AstDescription;
use crate::AstDirectivesConst;
use crate::AstLocation;
use crate::AstName;
use crate::AstNamedType;
use crate::AstType;

pub mod _3_6_1_field_arguments;
pub mod _3_6_3_object_extensions;

pub use _3_6_1_field_arguments::*;
pub use _3_6_3_object_extensions::*;

/// ObjectTypeDefinition :
///   `Description`? type `Name` `ImplementsInterfaces`? `Directives[Const]`? `FieldsDefinition`
///   `Description`? type `Name` `ImplementsInterfaces`? `Directives[Const]`? [lookahead != {\]
///
/// Spec: https://spec.graphql.org/draft/#ObjectTypeDefinition
#[derive(Clone, Debug)]
pub struct AstObjectTypeDefinition {
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

/// ImplementsInterfaces :
///   `ImplementsInterfaces` & `NamedType`
///   implements &? `NamedType`
///
/// Spec: https://spec.graphql.org/draft/#ImplementsInterfaces
#[derive(Clone, Debug)]
pub struct AstImplementsInterfaces {
	/// `ImplementsInterfaces` & `NamedType`
	/// implements &? `NamedType`
	pub implements: Vec<AstNamedType>,

	/// Node's location
	pub location: AstLocation,
}

/// FieldsDefinition :
///   { `FieldDefinition`+ }
///
/// Spec: https://spec.graphql.org/draft/#FieldsDefinition
#[derive(Clone, Debug)]
pub struct AstFieldsDefinition {
	/// `FieldDefinition`+
	pub fields: Vec<AstFieldDefinition>,

	/// Node's location
	pub location: AstLocation,
}

/// FieldDefinition:
///   `Description`? `Name` `ArgumentsDefinition`? : `Type` `Directives[Const]`?
///
/// Spec: https://spec.graphql.org/draft/#FieldDefinition
#[derive(Clone, Debug)]
pub struct AstFieldDefinition {
	/// `FieldDefinition`+
	pub description: Option<AstDescription>,

	/// `Name`
	pub name: AstName,

	/// `ArgumentsDefinition`?
	pub arguments: Option<AstArgumentsDefinition>,

	/// `Type`
	pub type_: AstType,

	/// `Directives[Const]`
	pub directives: Option<AstDirectivesConst>,

	/// Node's location
	pub location: AstLocation,
}
