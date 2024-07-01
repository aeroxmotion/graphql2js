use graphql_token::token::Token;

use super::Lexer;
use super::TokenResult;

/// Punctuator :: one of
///   !	$	&	(	)	...	:	=	@	[	]	{	|	}
///
/// Spec: https://spec.graphql.org/draft/#Punctuator
///
/// _Note_: Before calling make sure you import all the punctuators `use graphql_token::punctuator::*`
macro_rules! punctuator {
	() => {
		EXCLAMATION_MARK_PUNCTUATOR
			| DOLLAR_PUNCTUATOR
			| AMPERSAND_PUNCTUATOR
			| OPEN_PARENTHESIS_PUNCTUATOR
			| CLOSE_PARENTHESIS_PUNCTUATOR
			| COLON_PUNCTUATOR
			| EQUALS_PUNCTUATOR
			| AT_SIGN_PUNCTUATOR
			| OPENING_BRACKET_PUNCTUATOR
			| CLOSING_BRACKET_PUNCTUATOR
			| OPENING_BRACE_PUNCTUATOR
			| VERTICAL_BAR_PUNCTUATOR
			| CLOSING_BRACE_PUNCTUATOR
	};
}
pub(crate) use punctuator;

/// Parses any non-`Triple period punctuator` token.
///
/// Punctuator :: !	$	&	(	)	:	=	@	[	]	{	|	}
///
/// Spec: https://spec.graphql.org/draft/#Punctuator
pub(crate) fn try_parse_punctuator(lexer: &mut Lexer) -> TokenResult {
	let location = lexer.loc();
	let punctuator = lexer.peek();

	// Skip `<punctuator>`
	lexer.eat();

	Ok(Token::Punctuator(punctuator, location))
}

/// Parses `Triple period punctuator` token.
///
/// Punctuator :: ...
///
/// Spec: https://spec.graphql.org/draft/#Punctuator
pub(crate) fn try_parse_triple_period_punctuator(lexer: &mut Lexer) -> TokenResult {
	let location = lexer.loc();

	// Skip `...`
	lexer.eat_by(3);

	Ok(Token::Spread(location))
}
