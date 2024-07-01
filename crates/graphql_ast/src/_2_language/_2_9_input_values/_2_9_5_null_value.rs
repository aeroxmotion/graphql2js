use crate::location::AstLocation;

/// NullValue :
///   null
///
/// Spec: https://spec.graphql.org/draft/#NullValue
#[derive(Clone, Debug)]
pub struct AstNullValue {
	/// Node's location
	pub location: AstLocation,
}
