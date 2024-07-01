use crate::LexerState;
use graphql_common::result::GraphQLResult;
use graphql_token::Token;

/// Punctuator :: one of
///   ! $ & ( ) : = @ [ ] { | }
///
/// Spec: https://spec.graphql.org/draft/#Punctuator
pub fn try_read_punctuator(state: &mut LexerState) -> GraphQLResult<Token> {
	let location = state.loc();

	// `Punctuator`
	let punctuator = state.peek();

	// Skip `Punctuator`
	state.eat();

	Ok(Token::Punctuator(punctuator, location))
}

/// Punctuator :: ...
///
/// Spec: https://spec.graphql.org/draft/#Punctuator
pub fn try_read_spread_punctuator(state: &mut LexerState) -> GraphQLResult<Token> {
	let location = state.loc();

	// Skip ...
	state.eat_by(3);

	Ok(Token::Spread(location))
}
