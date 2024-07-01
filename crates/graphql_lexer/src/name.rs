use graphql_token::token::*;

use super::match_punctuator;
use super::Lexer;
use super::TokenResult;

/// Letter ::
///   A...Z
///   a...z
///
/// Spec: https://spec.graphql.org/draft/#Letter
macro_rules! letter {
	() => {
		'A'..='Z' | 'a'..='z'
	};
}
pub(crate) use letter;

/// Digit ::
///   0...9
///
/// Spec: https://spec.graphql.org/draft/#Digit
macro_rules! digit {
	() => {
		'0'..='9'
	};
}
pub(crate) use digit;

/// NameStart ::
///   `Letter`
///   _
///
/// Spec: https://spec.graphql.org/draft/#NameStart
macro_rules! name_start_pattern {
	() => {
		letter!() | '_'
	};
}
pub(crate) use name_start_pattern;

/// NameContinue ::
///   `Letter`
/// 	`Digit`
///   _
///
/// Spec: https://spec.graphql.org/draft/#NameContinue
macro_rules! name_continue_pattern {
	() => {
		letter!() | digit!() | '_'
	};
}

/// Parses `Name` token.
///
/// Name ::
///   `NameStart` `NameContinue` *list*,*opt* [lookahead != `NameContinue`\]
///
/// Spec: https://spec.graphql.org/draft/#Name
pub(crate) fn try_parse_name(lexer: &mut Lexer) -> TokenResult {
	let location = lexer.loc();
	let mut name = String::new();

	while {
		name.push(lexer.peek());
		lexer.eat();

		match_punctuator!(lexer, name_continue_pattern!())
	} {}

	Ok(Token::Name(name, location))
}
