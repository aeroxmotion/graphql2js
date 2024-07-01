use crate::location::AstLocation;

pub mod _3_1_type_extensions;
pub mod _3_2_descriptions;

pub use _3_1_type_extensions::*;
pub use _3_2_descriptions::*;

/// TypeSystemDocument :
///   `TypeSystemDefinition`+
///
/// Spec: https://spec.graphql.org/draft/#TypeSystemDocument
#[derive(Clone, Debug)]
pub struct AstTypeSystemDocument {
	/// `TypeSystemDefinition`+
	pub definitions: Vec<AstTypeSystemDefinition>,

	/// Node's location
	pub location: AstLocation,
}

/// TypeSystemDefinition :
///   `SchemaDefinition`
///   `TypeDefinition`
///   `DirectiveDefinition`
///
/// Spec: https://spec.graphql.org/draft/#TypeSystemDefinition
#[derive(Clone, Debug)]
pub enum AstTypeSystemDefinition {
	/// `SchemaDefinition`
	SchemaDefinition(AstSchemaDefinition),

	/// `TypeDefinition`
	TypeDefinition(AstTypeDefinition),

	/// `DirectiveDefinition`
	DirectiveDefintion(AstDirectiveDefinition),
}
