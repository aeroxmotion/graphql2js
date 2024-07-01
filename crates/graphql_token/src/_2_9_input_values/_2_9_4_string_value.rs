/// Spec: https://spec.graphql.org/draft/#StringValue
pub const DOUBLE_QUOTE: char = '"';

/// Spec: https://spec.graphql.org/draft/#StringCharacter
pub const BACKSLASH: char = '\\';

/// HexDigit :: one of
///   0...9 A...F a...f
///
/// Spec: https://spec.graphql.org/draft/#HexDigit
#[macro_export]
macro_rules! HexDigit {
	() => {
		'0'..='9' | 'A'..='F' | 'a'..='f'
	};
}
