use super::location::Location;

/// Defined in: https://spec.graphql.org/draft/#sec-Language.Source-Text.Lexical-Tokens
#[derive(Clone, Debug)]
pub enum Token {
	/// Start-Of-File <SOF>
	SOF,

	/// End-Of-File <EOF>
	EOF,

	/// Defined in: https://spec.graphql.org/draft/#sec-Punctuators
	Punctuator(char, Location),
	/// Defined in: https://spec.graphql.org/draft/#sec-Punctuators
	Spread(Location),

	/// Defined in: https://spec.graphql.org/draft/#sec-Names
	Name(String, Location),

	/// Defined in: https://spec.graphql.org/draft/#sec-Int-Value
	Int(String, Location),

	/// Defined in: https://spec.graphql.org/draft/#sec-Float-Value
	Float(String, Location),

	/// Defined in: https://spec.graphql.org/draft/#sec-String-Value
	String(String, Location),
	/// Defined in: https://spec.graphql.org/draft/#sec-String-Value
	BlockString(String, Location),
}
