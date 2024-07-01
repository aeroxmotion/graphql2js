use graphql_common::location::AstLocation;

/// StringValue : `StringValue`
///
/// Spec: https://spec.graphql.org/draft/#StringValue
#[derive(Clone, Debug)]
pub struct AstStringValue {
	/// `StringValue`
	pub value: String,

	/// `BlockString` representation
	pub block: bool,

	/// Node's location
	pub location: AstLocation,
}
