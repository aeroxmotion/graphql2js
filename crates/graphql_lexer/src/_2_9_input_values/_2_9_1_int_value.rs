use graphql_common::error::GraphQLError;
use graphql_common::error::GraphQLLexerError;
use graphql_common::result::GraphQLResult;
use graphql_token::Digit;
use graphql_token::NonZeroDigit;
use graphql_token::Token;
use graphql_token::NEGATIVE_SIGN;

use crate::try_read_float_value;
use crate::LexerState;

/// IntValue ::
///   `IntegerPart` [lookahead != {`Digit`, ., `NameStart`}\]
///
/// Spec: https://spec.graphql.org/draft/#IntValue
pub fn try_read_int_value(state: &mut LexerState) -> GraphQLResult<Token> {
	try_read_float_value(state)
}

/// IntegerPart ::
///   `NegativeSign`? 0
///   `NegativeSign`? `NonZeroDigit` `Digit`+?
///
/// Spec: https://spec.graphql.org/draft/#IntegerPart
pub fn try_read_integer_part(state: &mut LexerState) -> GraphQLResult<String> {
	let mut part = String::new();

	// `NegativeSign`?
	if state.is(NEGATIVE_SIGN) {
		part.push(NEGATIVE_SIGN);

		// Skip `NegativeSign`
		state.eat();
	}

	match state.try_peek()? {
		'0' => part.push('0'),
		NonZeroDigit!() => {
			while {
				// `NonZeroDigit` `Digit`+?
				part.push(state.peek());
				state.eat();

				matches!(state.peek(), Digit!())
			} {}
		}
		char => {
			return Err(GraphQLError::Lexer(GraphQLLexerError::InvalidToken(
				char,
				state.loc(),
			)))
		}
	}

	Ok(part)
}
