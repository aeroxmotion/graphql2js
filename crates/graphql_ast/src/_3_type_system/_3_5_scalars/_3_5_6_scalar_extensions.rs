use crate::AstDirectivesConst;
use crate::AstLocation;

/// ScalarTypeExtension :
///   extend scalar `Name` `Directives[Const]`
///
/// Spec: https://spec.graphql.org/draft/#ScalarTypeExtension
#[derive(Clone, Debug)]
pub struct AstScalarTypeExtension {
	/// `Directives[Const]`
	pub directives: AstDirectivesConst,

	/// Node's location
	pub location: AstLocation,
}
