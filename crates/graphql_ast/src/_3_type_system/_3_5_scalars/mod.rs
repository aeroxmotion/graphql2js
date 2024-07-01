use crate::AstDescription;
use crate::AstDirectivesConst;
use crate::AstLocation;
use crate::AstName;

pub mod _3_5_6_scalar_extensions;

pub use _3_5_6_scalar_extensions::*;

/// ScalarTypeDefinition :
///   `Description`? scalar `Name` `Directives[Const]`?
///
/// Spec: https://spec.graphql.org/draft/#ScalarTypeDefinition
pub struct AstScalarTypeDefinition {
	/// `Description`?
	pub description: Option<AstDescription>,

	/// `Name`
	pub name: AstName,

	/// `Directives[Const]`?
	pub directives: Option<AstDirectivesConst>,

	/// Node's location
	pub location: AstLocation,
}