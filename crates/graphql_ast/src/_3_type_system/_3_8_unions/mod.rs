use graphql_common::location::AstLocation;

use crate::AstDescription;
use crate::AstDirectivesConst;
use crate::AstName;
use crate::AstNamedType;

pub mod _3_8_1_union_extensions;

pub use _3_8_1_union_extensions::*;

/// UnionTypeDefinition :
///   `Description`? union `Name` `Directives[Const]`? `UnionMemberTypes`?
///
/// Spec: https://spec.graphql.org/draft/#UnionTypeDefinition
#[derive(Clone, Debug)]
pub struct AstUnionTypeDefinition {
	/// `Description`?
	pub description: Option<AstDescription>,

	/// `Name`
	pub name: AstName,

	/// `Directives[Const]`?
	pub directives: Option<AstDirectivesConst>,

	/// Node's location
	pub location: AstLocation,
}

/// UnionMemberTypes :
///   `UnionMemberTypes` | `NamedType`
///   = |? `NamedType`
///
/// Spec: https://spec.graphql.org/draft/#UnionMemberTypes
#[derive(Clone, Debug)]
pub struct AstUnionMemberTypes {
	/// `UnionMemberTypes` | `NamedType`
	/// = |? `NamedType`
	pub members: Vec<AstNamedType>,

	/// Node's location
	pub location: AstLocation,
}
