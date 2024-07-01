/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const EXCLAMATION_MARK_PUNCTUATOR: char = '!';

/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const DOLLAR_SIGN_PUNCTUATOR: char = '$';

/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const AMPERSAND_SIGN_PUNCTUATOR: char = '&';

/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const OPEN_PAREN_PUNCTUATOR: char = '(';

/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const CLOSE_PAREN_PUNCTUATOR: char = ')';

/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const PERIOD_PUNCTUATOR: char = '.';

/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const COLON_PUNCTUATOR: char = ':';

/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const EQUALS_PUNCTUATOR: char = '=';

/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const AT_SIGN_PUNCTUATOR: char = '@';

/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const OPEN_BRACKET_PUNCTUATOR: char = '[';

/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const CLOSE_BRACKET_PUNCTUATOR: char = ']';

/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const OPEN_BRACE_PUNCTUATOR: char = '{';

/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const VERTICAL_BAR_PUNCTUATOR: char = '|';

/// Spec: https://spec.graphql.org/draft/#Punctuator
pub const CLOSE_BRACE_PUNCTUATOR: char = '}';

/// Punctuator :: one of
///   ! $ & ( ) ... : = @ [ ] { | }
///
/// Spec: https://spec.graphql.org/draft/#Punctuator
#[macro_export]
macro_rules! Punctuator {
	() => {
		EXCLAMATION_MARK_PUNCTUATOR
			| DOLLAR_SIGN_PUNCTUATOR
			| AMPERSAND_SIGN_PUNCTUATOR
			| OPEN_PAREN_PUNCTUATOR
			| CLOSE_PAREN_PUNCTUATOR
			| COLON_PUNCTUATOR
			| EQUALS_PUNCTUATOR
			| AT_SIGN_PUNCTUATOR
			| OPEN_BRACKET_PUNCTUATOR
			| CLOSE_BRACKET_PUNCTUATOR
			| OPEN_BRACE_PUNCTUATOR
			| VERTICAL_BAR_PUNCTUATOR
			| CLOSE_BRACE_PUNCTUATOR
	};
}
