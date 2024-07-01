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
