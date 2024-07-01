use crate::location::AstLocation;

/// FloatValue : `FloatValue`
///
/// Spec: https://spec.graphql.org/draft/#FloatValue
#[derive(Clone, Debug)]
pub struct AstFloatValue {
	/// Evaluated raw's `FloatValue` representation
	pub value: f64,

	/// `FloatValue`
	pub raw: String,

	/// Node's location
	pub location: AstLocation,
}
