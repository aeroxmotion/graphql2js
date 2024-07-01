use crate::AstEnumTypeExtension;
use crate::AstInputObjectTypeExtension;
use crate::AstInterfaceTypeExtension;
use crate::AstObjectTypeExtension;
use crate::AstScalarTypeExtension;
use crate::AstUnionTypeExtension;

/// TypeExtension :
///   `ScalarTypeExtension`
///   `ObjectTypeExtension`
///   `InterfaceTypeExtension`
///   `UnionTypeExtension`
///   `EnumTypeExtension`
///   `InputObjectTypeExtension`
///
/// Spec: https://spec.graphql.org/draft/#TypeExtension
#[derive(Clone, Debug)]
pub enum AstTypeExtension {
	/// `ScalarTypeExtension`
	ScalarTypeExtension(AstScalarTypeExtension),

	/// `ObjectTypeExtension`
	ObjectTypeExtension(AstObjectTypeExtension),

	/// `InterfaceTypeExtension`
	InterfaceTypeExtension(AstInterfaceTypeExtension),

	/// `UnionTypeExtension`
	UnionTypeExtension(AstUnionTypeExtension),

	/// `EnumTypeExtension`
	EnumTypeExtension(AstEnumTypeExtension),

	/// `InputObjectTypeExtension`
	InputObjectTypeExtension(AstInputObjectTypeExtension),
}
