use graphql_common::error::GraphQLError;
use graphql_common::error::GraphQLLexerError;
use graphql_common::result::GraphQLResult;
use graphql_token::*;

use crate::try_read_block_string;
use crate::try_read_float_value;
use crate::try_read_name;
use crate::try_read_punctuator;
use crate::try_read_spread_punctuator;
use crate::try_read_string_value;
use crate::LexerState;

/// Token ::
///   `Punctuator`
///   `Name`
///   `IntValue`
///   `FloatValue`
///   `StringValue`
///
/// Spec: https://spec.graphql.org/draft/#Token
pub fn try_read_token(state: &mut LexerState) -> GraphQLResult<Token> {
	'root: while !state.end() {
		return match state.peek() {
			// Ignored :: one of
			//   UnicodeBOM WhiteSpace Comma
			UNICODE_BOM | WhiteSpace!() | COMMA => {
				state.eat();
				continue;
			}

			// Ignored ::
			//   LineTerminator ::
			//     `Carriage Return` `New Line`
			CARRIAGE_RETURN if state.is_by(NEW_LINE, 1) => {
				state.eat();
				state.eat_line();
				continue;
			}

			// Ignored ::
			//   LineTerminator ::
			//     `New Line`
			//     `Carriage Return` [lookahead != New Line]
			LineTerminator!() => {
				state.eat_line();
				continue;
			}

			// Ignored ::
			//   Comment ::
			//     # `CommentChar`+? [lookahead != `CommentChar`]
			//
			//   CommentChar ::
			// 		   `SourceCharacter` but not `LineTerminator`
			NUMBER_SIGN => {
				loop {
					// Skip `CommentChar`
					state.eat();

					// Break if ended
					if state.end() {
						break 'root;
					}

					// Skip until encounter `LF` or `CR`
					if matches!(state.peek(), LineTerminator!()) {
						continue 'root;
					}
				}
			}

			// `Punctuator`
			Punctuator!() => try_read_punctuator(state),
			PERIOD_PUNCTUATOR if state.lookahead(PERIOD_PUNCTUATOR, 2) => {
				try_read_spread_punctuator(state)
			}

			// `Name`
			NameStart!() => try_read_name(state),

			// `IntValue`
			// `FloatValue`
			NEGATIVE_SIGN => try_read_float_value(state),
			Digit!() => try_read_float_value(state),

			// `StringValue`
			DOUBLE_QUOTE => {
				if state.lookahead(DOUBLE_QUOTE, 2) {
					try_read_block_string(state)
				} else {
					try_read_string_value(state)
				}
			}

			// <Unknown token>
			char => Err(GraphQLError::Lexer(GraphQLLexerError::UnknownToken(
				char,
				state.loc(),
			))),
		};
	}

	Ok(Token::EOF)
}
