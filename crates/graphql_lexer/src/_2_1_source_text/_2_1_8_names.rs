use graphql_common::result::GraphQLResult;
use graphql_token::*;

use crate::LexerState;

/// Name ::
///   `NameStart` `NameContinue`+? [lookahead != `NameContinue`]
///
/// Spec: https://spec.graphql.org/draft/#Name
pub fn try_read_name(state: &mut LexerState) -> GraphQLResult<Token> {
	let location = state.loc();
	let mut name = String::new();

	while {
		// `NameStart` `NameContinue`+?
		name.push(state.peek());
		state.eat();

		// [lookahead != `NameContinue`]
		matches!(state.peek(), NameContinue!())
	} {}

	Ok(Token::Name(name, location))
}
