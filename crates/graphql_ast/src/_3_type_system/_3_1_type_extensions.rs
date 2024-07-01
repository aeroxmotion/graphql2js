use super::AstTypeSystemDefinition;
use crate::location::AstLocation;

/// TypeSystemExtensionDocument :
///   `TypeSystemDefinitionOrExtension`+
///
/// Spec: https://spec.graphql.org/draft/#TypeSystemExtensionDocument
#[derive(Clone, Debug)]
pub struct AstTypeSystemExtensionDocument {
	/// `TypeSystemDefinitionOrExtension`+
	pub definitions_or_extensions: Vec<AstTypeSystemDefinitionOrExtension>,

	/// Node's location
	pub location: AstLocation,
}

/// TypeSystemDefinitionOrExtension :
///   `TypeSystemDefinition`
///   `TypeSystemExtension`
///
/// Spec: https://spec.graphql.org/draft/#TypeSystemDefinitionOrExtension
#[derive(Clone, Debug)]
pub enum AstTypeSystemDefinitionOrExtension {
	/// `TypeSystemDefinition`
	TypeSystemDefinition(AstTypeSystemDefinition),

	/// `TypeSystemExtension`
	TypeSystemExtension(AstTypeSystemExtension),
}

/// TypeSystemExtension :
///   `SchemaExtension`
///   `TypeExtension`
///
/// Spec: https://spec.graphql.org/draft/#TypeSystemExtension
#[derive(Clone, Debug)]
pub enum AstTypeSystemExtension {
	/// `SchemaExtension`
	SchemaExtension(AstSchemaExtension),

	/// `TypeExtension`
	TypeExtension(AstTypeExtension),
}
