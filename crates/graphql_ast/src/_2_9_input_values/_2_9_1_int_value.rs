use crate::location::AstLocation;

/// IntValue : `IntValue`
///
/// Spec: https://spec.graphql.org/draft/#IntValue
#[derive(Clone, Debug)]
pub struct AstIntValue {
	/// Evaluated raw's `IntValue` representation
	pub value: isize,

	/// `IntValue`
	pub raw: String,

	/// Node's location
	pub location: AstLocation,
}
