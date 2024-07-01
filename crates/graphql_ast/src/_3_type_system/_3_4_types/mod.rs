use crate::AstEnumTypeDefinition;
use crate::AstInputObjectTypeDefinition;
use crate::AstInterfaceTypeDefinition;
use crate::AstObjectTypeDefinition;
use crate::AstScalarTypeDefinition;
use crate::AstUnionTypeDefinition;

pub mod _3_4_3_type_extensions;

pub use _3_4_3_type_extensions::*;

/// TypeDefinition :
///   `ScalarTypeDefinition`
///   `ObjectTypeDefinition`
///   `InterfaceTypeDefinition`
///   `UnionTypeDefinition`
///   `EnumTypeDefinition`
///   `InputObjectTypeDefinition`
///
/// Spec: https://spec.graphql.org/draft/#TypeDefinition
#[derive(Clone, Debug)]
pub enum AstTypeDefinition {
	/// `ScalarTypeDefinition`
	ScalarTypeDefinition(AstScalarTypeDefinition),

	/// `ObjectTypeDefinition`
	ObjectTypeDefinition(AstObjectTypeDefinition),

	/// `InterfaceTypeDefinition`
	InterfaceTypeDefinition(AstInterfaceTypeDefinition),

	/// `UnionTypeDefinition`
	UnionTypeDefinition(AstUnionTypeDefinition),

	/// `EnumTypeDefinition`
	EnumTypeDefinition(AstEnumTypeDefinition),

	/// `InputObjectTypeDefinition`
	InputObjectTypeDefinition(AstInputObjectTypeDefinition),
}
