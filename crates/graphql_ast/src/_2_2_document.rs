use crate::_2_3_operations::AstOperationDefinition;
use crate::_2_8_fragments::AstFragmentDefinition;
use crate::_3_1_type_extensions::AstTypeSystemDefinitionOrExtension;
use crate::location::AstLocation;

/// Document :
///   `Definition`+
///
/// Spec: https://spec.graphql.org/draft/#Document
#[derive(Clone, Debug)]
pub struct AstDocument {
	/// `Definition`+
	pub definitions: Vec<AstDefinition>,

	/// Node's location
	pub location: AstLocation,
}

/// Definition :
///   `ExecutableDefinition`
///   `TypeSystemDefinitionOrExtension`
///
/// Spec: https://spec.graphql.org/draft/#Definition
#[derive(Clone, Debug)]
pub enum AstDefinition {
	/// `ExecutableDefinition`
	ExecutableDefinition(AstExecutableDefinition),

	/// `TypeSystemDefinitionOrExtension`
	TypeSystemDefinitionOrExtension(AstTypeSystemDefinitionOrExtension),
}

/// ExecutableDocument :
///   `ExecutableDefinition`+
///
/// Spec: https://spec.graphql.org/draft/#ExecutableDocument
#[derive(Clone, Debug)]
pub struct AstExecutableDocument {
	/// `ExecutableDefinition`+
	pub definitions: Vec<AstExecutableDefinition>,

	/// Node's location
	pub location: AstLocation,
}

/// ExecutableDefinition :
///   `OperationDefinition`
///   `FragmentDefinition`
///
/// Spec: https://spec.graphql.org/draft/#ExecutableDefinition
#[derive(Clone, Debug)]
pub enum AstExecutableDefinition {
	/// `OperationDefinition`
	OperationDefinition(AstOperationDefinition),

	/// `FragmentDefinition`
	FragmentDefinition(AstFragmentDefinition),
}
