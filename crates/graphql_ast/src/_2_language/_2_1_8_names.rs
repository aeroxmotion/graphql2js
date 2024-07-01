use crate::location::AstLocation;

/// Name : `Name`
///
/// Spec: https://spec.graphql.org/draft/#Name
#[derive(Clone, Debug)]
pub struct AstName {
	/// `Name`
	pub name: String,

	/// Node's location
	pub location: AstLocation,
}
