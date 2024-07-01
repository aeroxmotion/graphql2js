use crate::location::AstLocation;

/// BooleanValue : one of
///   true false
///
/// Spec: https://spec.graphql.org/draft/#BooleanValue
#[derive(Clone, Debug)]
pub struct AstBooleanValue {
	/// Evaluated raw's `BooleanValue` representation
	pub value: bool,

	/// `BooleanValue`
	pub raw: String,

	/// Node's location
	pub location: AstLocation,
}
