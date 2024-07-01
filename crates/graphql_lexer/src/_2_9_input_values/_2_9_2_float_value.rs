use crate::try_read_integer_part;
use crate::LexerState;
use graphql_common::error::GraphQLError;
use graphql_common::error::GraphQLLexerError;
use graphql_common::result::GraphQLResult;
use graphql_token::Digit;
use graphql_token::ExponentIndicator;
use graphql_token::Sign;
use graphql_token::Token;

/// FloatValue ::
///   `IntegerPart` `FractionalPart` `ExponentPart` [lookahead != {`Digit`, ., `NameStart`}\]
///   `IntegerPart` `FractionalPart` [lookahead != {`Digit`, ., `NameStart`}\]
///   `IntegerPart` `ExponentPart` [lookahead != {`Digit`, ., `NameStart`}\]
///
/// Spec: https://spec.graphql.org/draft/#FloatValue
pub fn try_read_float_value(state: &mut LexerState) -> GraphQLResult<Token> {
	let location = state.loc();

	// `IntegerPart`
	let integer_part = try_read_integer_part(state)?;

	// `FractionalPart`
	let fractional_part = try_read_fractional_part(state)?;

	// `ExponentPart`
	let exponent_part = try_read_exponent_part(state)?;

	let value = format!("{integer_part}{fractional_part}{exponent_part}");

	Ok(if fractional_part.len().min(exponent_part.len()) > 0 {
		Token::FloatValue(value, location)
	} else {
		Token::IntValue(value, location)
	})
}

/// FractionalPart ::
///   . `Digit`+
///
/// Spec: https://spec.graphql.org/draft/#FractionalPart
pub fn try_read_fractional_part(state: &mut LexerState) -> GraphQLResult<String> {
	let mut part = String::new();

	// .
	if state.is('.') {
		// Skip .
		state.eat();

		// `Digit`
		match state.try_peek()? {
			digit @ (Digit!()) => {
				// `Digit`
				part.push(digit);
				state.eat();
			}
			char => {
				return Err(GraphQLError::Lexer(GraphQLLexerError::InvalidToken(
					char,
					state.loc(),
				)))
			}
		}

		// `Digit`+?
		while !state.end() && matches!(state.peek(), Digit!()) {
			part.push(state.peek());
			state.eat();
		}
	}

	Ok(part)
}

/// ExponentPart ::
///   `ExponentIndicator` `Sign`? `Digit`+
///
/// Spec: https://spec.graphql.org/draft/#ExponentPart
pub fn try_read_exponent_part(state: &mut LexerState) -> GraphQLResult<String> {
	let mut part = String::new();

	// `ExponentIndicator`
	if !state.end() && matches!(state.peek(), ExponentIndicator!()) {
		state.eat();

		// `Sign`?
		if !state.end() && matches!(state.peek(), Sign!()) {
			part.push(state.peek());
			state.eat();
		}

		// `Digit`
		match state.try_peek()? {
			digit @ (Digit!()) => {
				// `Digit`
				part.push(digit);
				state.eat();
			}
			char => {
				return Err(GraphQLError::Lexer(GraphQLLexerError::InvalidToken(
					char,
					state.loc(),
				)))
			}
		}

		// `Digit`+?
		while !state.end() && matches!(state.peek(), Digit!()) {
			part.push(state.peek());
			state.eat();
		}
	}

	Ok(part)
}
