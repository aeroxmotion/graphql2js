use crate::AstDirectivesConst;
use crate::AstLocation;
use crate::AstName;
use crate::AstUnionMemberTypes;

/// UnionTypeExtension :
///   extend union `Name` `Directives[Const]`? `UnionMemberTypes`
///   extend union `Name` `Directives[Const]`
///
/// Spec: https://spec.graphql.org/draft/#UnionTypeExtension
#[derive(Clone, Debug)]
pub struct AstUnionTypeExtension {
	/// `Name`
	pub name: AstName,

	/// `Directives[Const]`?
	pub directives: Option<AstDirectivesConst>,

	/// `UnionMemberTypes`
	/// <abscense>
	pub members: Option<AstUnionMemberTypes>,

	/// Node's location
	pub location: AstLocation,
}
