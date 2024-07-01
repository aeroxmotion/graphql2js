use crate::Location;

/// Token ::
///   `Punctuator`
///   `Name`
///   `IntValue`
///   `FloatValue`
///   `StringValue`
///
/// Spec: https://spec.graphql.org/draft/#Token
pub enum Token {
	/// `Punctuator`
	Punctuator(char, Location),
	Spread(Location),

	/// `Name`
	Name(String, Location),

	/// `IntValue`
	IntValue(String, Location),

	/// `FloatValue`
	FloatValue(String, Location),

	/// `StringValue`
	StringValue(String, Location),
	BlockString(String, Location),
}
