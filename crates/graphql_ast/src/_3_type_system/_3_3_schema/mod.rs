use crate::AstDescription;
use crate::AstDirectivesConst;
use crate::AstNamedType;
use crate::AstOperationType;

pub mod _3_3_2_schema_extension;

pub use _3_3_2_schema_extension::*;

/// SchemaDefinition :
///   `Description`? schema `Directives[Const]`? { `RootOperationTypeDefinition`+ }
///
/// Spec: https://spec.graphql.org/draft/#SchemaDefinition
#[derive(Clone, Debug)]
pub struct AstSchemaDefinition {
	/// `Description`?
	pub description: Option<AstDescription>,

	/// `Directives`?
	pub directives: Option<AstDirectivesConst>,

	/// `RootOperationTypeDefinition`+
	pub definitions: Vec<AstRootOperationTypeDefinition>,
}

/// RootOperationTypeDefinition :
///   `OperationType` : `NamedType`
///
/// Spec: https://spec.graphql.org/draft/#RootOperationTypeDefinition
#[derive(Clone, Debug)]
pub struct AstRootOperationTypeDefinition {
	/// `OperationType`
	pub operation_type: AstOperationType,

	/// `NamedType`
	pub type_: AstNamedType,
}
