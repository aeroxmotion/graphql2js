use graphql_common::location::TokenLocation;

/// Token ::
///   `Punctuator`
///   `Name`
///   `IntValue`
///   `FloatValue`
///   `StringValue`
///
/// Spec: https://spec.graphql.org/draft/#Token
pub enum Token {
	/// End-Of-File
	EOF,

	/// `Punctuator`
	Punctuator(char, TokenLocation),
	Spread(TokenLocation),

	/// `Name`
	Name(String, TokenLocation),

	/// `IntValue`
	IntValue(String, TokenLocation),

	/// `FloatValue`
	FloatValue(String, TokenLocation),

	/// `StringValue`
	StringValue(String, TokenLocation),
	BlockString(String, TokenLocation),
}
