use crate::_2_1_8_names::AstName;
use crate::location::AstLocation;

/// Type :
///   `NamedType`
///   `ListType`
///   `NonNullType`
///
/// Spec: https://spec.graphql.org/draft/#Type
#[derive(Clone, Debug)]
pub enum AstType {
	/// `NamedType`
	NamedType(AstNamedType),

	/// `ListType`
	ListType(Box<AstListType>),

	/// `NonNullType`
	NonNullType(),
}

/// NamedType :
///   `Name`
///
/// Spec: https://spec.graphql.org/draft/#NamedType
pub type AstNamedType = AstName;

/// ListType :
///   [ `Type` ]
///
/// Spec: https://spec.graphql.org/draft/#ListType
#[derive(Clone, Debug)]
pub struct AstListType {
	/// `Type`
	pub type_: AstType,

	/// Node's location
	pub location: AstLocation,
}

/// NonNullType :
///   `NamedType`
///   `ListType`
///
/// Spec: https://spec.graphql.org/draft/#NonNullType
#[derive(Clone, Debug)]
pub enum AstNonNullType {
	/// `NamedType`
	NamedType(AstNamedType),

	/// `ListType`
	ListType(Box<AstListType>),
}
