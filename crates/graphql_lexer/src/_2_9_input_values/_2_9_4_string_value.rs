use graphql_common::error::GraphQLError;
use graphql_common::error::GraphQLLexerError;
use graphql_common::result::GraphQLResult;
use graphql_token::*;

use super::_2_9_4_string_value_static_semantics::*;
use crate::LexerState;

/// StringValue ::
///   " " [lookahead != "]
///   " StringCharacter+ "
///
/// Spec: https://spec.graphql.org/draft/#StringValue
pub fn try_read_string_value(state: &mut LexerState) -> GraphQLResult<Token> {
	let location = state.loc();

	// Skip leading `"`
	state.eat();

	let string_value = if state.is(DOUBLE_QUOTE) {
		static_semantic_string_value_empty_sequence()
	} else {
		let mut value = String::new();

		while {
			value += &try_read_string_character(state)?;

			!state.is(DOUBLE_QUOTE)
		} {}

		static_semantic_string_character_list(value)
	};

	// Skip trailing `"`
	state.eat();

	Ok(Token::StringValue(string_value, location))
}

/// StringCharacter ::
///   `SourceCharacter` but not " or \ or `LineTerminator`
///   \u `EscapedUnicode`
///   \ `EscapedCharacter`
///
/// Spec: https://spec.graphql.org/draft/#StringCharacter
fn try_read_string_character(state: &mut LexerState) -> GraphQLResult<String> {
	match state.try_peek()? {
		line_terminator @ (LineTerminator!()) => Err(GraphQLError::Lexer(
			GraphQLLexerError::InvalidToken(line_terminator, state.loc()),
		)),
		BACKSLASH => {
			// StringCharacter :: \u `EscapedUnicode`
			if state.lookahead('u', 1) {
				try_read_escaped_unicode(state)

			// StringCharacter :: \ `EscapedCharacter`
			} else {
				try_read_escaped_character(state)
			}
		}
		// StringCharacter :: `SourceCharacter` but not " or \ or `LineTerminator`
		char => {
			state.eat();

			Ok(static_semantic_string_character(char).into())
		}
	}
}

/// EscapedUnicode ::
///   { `HexDigit`+ }
///   `HexDigit` `HexDigit` `HexDigit` `HexDigit`
///
/// Spec: https://spec.graphql.org/draft/#EscapedUnicode
fn try_read_escaped_unicode(state: &mut LexerState) -> GraphQLResult<String> {
	Ok(if state.is_by(OPEN_BRACE_PUNCTUATOR, 3) {
		static_semantic_escaped_unicode(try_read_variable_width_escaped_unicode(state)?).into()
	} else {
		let leading = try_read_fixed_width_escaped_unicode(state)?;

		if state.is(BACKSLASH) && state.is_by('u', 1) && !state.is_by(OPEN_BRACE_PUNCTUATOR, 2) {
			static_semantic_possible_unicode_surrogate_pair(
				leading,
				try_read_fixed_width_escaped_unicode(state)?,
			)
		} else {
			static_semantic_escaped_unicode(leading).into()
		}
	})
}

/// EscapedUnicode ::
///   { `HexDigit` *list* }
///
/// Spec: https://spec.graphql.org/draft/#EscapedUnicode
fn try_read_variable_width_escaped_unicode(state: &mut LexerState) -> GraphQLResult<String> {
	// Skip `\u{`
	state.eat_by(3);

	// Ensure at least one hex digit
	let mut unicode = String::from(try_read_hex_digit(state)?);

	while !state.try_is(CLOSE_BRACE_PUNCTUATOR)? {
		unicode.push(try_read_hex_digit(state)?);
	}

	// Skip trailing `}`
	state.eat();

	Ok(unicode)
}

/// EscapedUnicode :: `HexDigit` `HexDigit` `HexDigit` `HexDigit`
///
/// Spec: https://spec.graphql.org/draft/#EscapedUnicode
fn try_read_fixed_width_escaped_unicode(state: &mut LexerState) -> GraphQLResult<String> {
	// Skip `\u`
	state.eat_by(2);

	let mut unicode = String::new();

	for _ in 0..4 {
		unicode.push(try_read_hex_digit(state)?);
	}

	Ok(unicode)
}

/// Parses `EscapedCharacter`.
///
/// EscapedCharacter :: one of
///   " \ / b f n r t
///
/// Spec: https://spec.graphql.org/draft/#EscapedCharacter
fn try_read_escaped_character(state: &mut LexerState) -> GraphQLResult<String> {
	// Skip `\`
	state.eat();

	let char = state.try_peek()?;
	let scalar_value = if let Some(value) = static_semantic_escaped_character(char) {
		value
	} else {
		return Err(GraphQLError::Lexer(GraphQLLexerError::InvalidToken(
			char,
			state.loc(),
		)));
	};

	// Skip `<char>`
	state.eat();

	Ok(scalar_value.into())
}

/// HexDigit
///
/// Spec: https://spec.graphql.org/draft/#HexDigit
fn try_read_hex_digit(state: &mut LexerState) -> GraphQLResult<char> {
	match state.try_peek()? {
		hex_digit @ (HexDigit!()) => {
			state.eat();

			Ok(hex_digit)
		}
		char => Err(GraphQLError::Lexer(GraphQLLexerError::InvalidToken(
			char,
			state.loc(),
		))),
	}
}

/// BlockString ::
///   """ `BlockStringCharacter` *list*,*opt* """
///
/// Spec: https://spec.graphql.org/draft/#BlockString
pub fn try_read_block_string(state: &mut LexerState) -> GraphQLResult<Token> {
	let location = state.loc();

	// Skip leading `"""`
	state.eat_by(3);

	let mut raw_value = String::new();

	while !state.try_is(DOUBLE_QUOTE)? || !state.lookahead(DOUBLE_QUOTE, 2) {
		raw_value.push_str(read_block_string_character(state).as_str());
	}

	// Skip trailing `"""`
	state.eat_by(3);

	Ok(Token::BlockString(
		static_semantic_block_string_character_list(raw_value),
		location,
	))
}

/// BlockStringCharacter ::
///   `SourceCharacter` but not """ or \"""
///   \"""
///
/// Spec: https://spec.graphql.org/draft/#BlockStringCharacter
fn read_block_string_character(state: &mut LexerState) -> String {
	match state.peek() {
		// BlockStringCharacter :: \"""
		BACKSLASH if state.lookahead(DOUBLE_QUOTE, 3) => {
			// Skip `\"""`
			state.eat_by(4);

			static_semantic_escaped_block_string_terminator()
		}
		CARRIAGE_RETURN if state.is_by(NEW_LINE, 1) => {
			// Skip `<CR><LF>`
			state.eat();
			state.eat_line();

			format!("{CARRIAGE_RETURN}{NEW_LINE}")
		}
		line_terminator @ (LineTerminator!()) => {
			state.eat_line();
			line_terminator.into()
		}
		// BlockStringCharacter :: `SourceCharacter` but not """ or \"""
		char => {
			state.eat();

			static_semantic_block_string_character(char).into()
		}
	}
}
