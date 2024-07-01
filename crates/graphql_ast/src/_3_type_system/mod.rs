use crate::location::AstLocation;

pub mod _3_1_type_extensions;
pub mod _3_2_descriptions;
pub mod _3_3_schema;
pub mod _3_4_types;
pub mod _3_5_scalars;
pub mod _3_6_objects;
pub mod _3_7_interfaces;
pub mod _3_8_unions;

pub use _3_1_type_extensions::*;
pub use _3_2_descriptions::*;
pub use _3_3_schema::*;
pub use _3_4_types::*;
pub use _3_5_scalars::*;
pub use _3_6_objects::*;
pub use _3_7_interfaces::*;
pub use _3_8_unions::*;

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
