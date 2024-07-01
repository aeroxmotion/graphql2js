use graphql_token::punctuator::*;

use super::error::LexerError;
use super::ignored::*;
use super::static_semantics::*;
use super::Lexer;
use super::Token;
use super::TokenResult;

/// Spec: https://spec.graphql.org/draft/#StringValue
pub(crate) const BACKSLASH: char = '\\';

/// Spec: https://spec.graphql.org/draft/#StringValue
const U_LETTER_CHAR: char = 'u';

/// Spec: https://spec.graphql.org/draft/#StringValue
pub(crate) const DOUBLE_QUOTE: char = '"';

/// Parses `StringValue` token.
///
/// StringValue ::
///   - " " [lookahead != "]
///   - " StringCharacter+ "
///
/// Spec: https://spec.graphql.org/draft/#StringValue
pub(crate) fn try_parse_string_value(lexer: &mut Lexer) -> TokenResult {
	let location = lexer.loc();

	// Skip leading `"`
	lexer.eat();

	let string_value = if lexer.is(DOUBLE_QUOTE) {
		static_semantic_string_value_empty_sequence()
	} else {
		let mut value = String::new();

		while {
			value += &try_parse_string_character(lexer)?;

			!lexer.is(DOUBLE_QUOTE)
		} {}

		static_semantic_string_character_list(value)
	};

	// Skip trailing `"`
	lexer.eat();

	Ok(Token::String(string_value, location))
}

/// StringCharacter ::
///   `SourceCharacter` but not " or \ or `LineTerminator`
///   \u `EscapedUnicode`
///   \ `EscapedCharacter`
///
/// Spec: https://spec.graphql.org/draft/#StringCharacter
fn try_parse_string_character(lexer: &mut Lexer) -> Result<String, LexerError> {
	match lexer.try_peek()? {
		line_terminator @ (line_terminator!()) => {
			Err(LexerError::InvalidToken(line_terminator, lexer.loc()))
		}
		BACKSLASH => {
			// StringCharacter :: \u `EscapedUnicode`
			if lexer.lookahead(U_LETTER_CHAR, 1) {
				try_parse_escaped_unicode(lexer)

			// StringCharacter :: \ `EscapedCharacter`
			} else {
				try_parse_escaped_character(lexer)
			}
		}
		// StringCharacter :: `SourceCharacter` but not " or \ or `LineTerminator`
		char => {
			lexer.eat();

			Ok(static_semantic_string_character(char).into())
		}
	}
}

/// HexDigit :: one of
///   0...9
///   A...F
///   a...f
///
/// Spec: https://spec.graphql.org/draft/#HexDigit
macro_rules! hex_digit {
	() => {
		'0'..='9' | 'A'..='F' | 'a'..='f'
	};
}

/// EscapedUnicode ::
///   { `HexDigit` *list* }
///   `HexDigit` `HexDigit` `HexDigit` `HexDigit`
///
/// Spec: https://spec.graphql.org/draft/#EscapedUnicode
fn try_parse_escaped_unicode(lexer: &mut Lexer) -> Result<String, LexerError> {
	Ok(if lexer.is_by(OPENING_BRACE_PUNCTUATOR, 3) {
		static_semantic_escaped_unicode(try_parse_variable_width_escaped_unicode(lexer)?).into()
	} else {
		let leading = try_parse_fixed_width_escaped_unicode(lexer)?;

		if lexer.is(BACKSLASH)
			&& lexer.is_by(U_LETTER_CHAR, 1)
			&& !lexer.is_by(OPENING_BRACE_PUNCTUATOR, 2)
		{
			static_semantic_possible_unicode_surrogate_pair(
				leading,
				try_parse_fixed_width_escaped_unicode(lexer)?,
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
fn try_parse_variable_width_escaped_unicode(lexer: &mut Lexer) -> Result<String, LexerError> {
	// Skip `\u{`
	lexer.eat_by(3);

	// Ensure at least one hex digit
	let mut unicode = String::from(try_parse_hex_digit(lexer)?);

	while !lexer.try_is(CLOSING_BRACE_PUNCTUATOR)? {
		unicode.push(try_parse_hex_digit(lexer)?);
	}

	// Skip trailing `}`
	lexer.eat();

	Ok(unicode)
}

/// EscapedUnicode :: `HexDigit` `HexDigit` `HexDigit` `HexDigit`
///
/// Spec: https://spec.graphql.org/draft/#EscapedUnicode
fn try_parse_fixed_width_escaped_unicode(lexer: &mut Lexer) -> Result<String, LexerError> {
	// Skip `\u`
	lexer.eat_by(2);

	let mut unicode = String::new();

	for _ in 0..4 {
		unicode.push(try_parse_hex_digit(lexer)?);
	}

	Ok(unicode)
}

/// Parses `EscapedCharacter`.
///
/// EscapedCharacter :: one of
///   " \ / b f n r t
///
/// Spec: https://spec.graphql.org/draft/#EscapedCharacter
fn try_parse_escaped_character(lexer: &mut Lexer) -> Result<String, LexerError> {
	// Skip `\`
	lexer.eat();

	let char = lexer.try_peek()?;
	let scalar_value = if let Some(value) = static_semantic_escaped_character(char) {
		value
	} else {
		return Err(LexerError::InvalidToken(char, lexer.loc()));
	};

	// Skip `<char>`
	lexer.eat();

	Ok(scalar_value.into())
}

/// Parses `HexDigit` token.
///
/// Spec: https://spec.graphql.org/draft/#HexDigit
fn try_parse_hex_digit(lexer: &mut Lexer) -> Result<char, LexerError> {
	match lexer.try_peek()? {
		hex_digit @ (hex_digit!()) => {
			lexer.eat();

			Ok(hex_digit)
		}
		char => Err(LexerError::InvalidToken(char, lexer.loc())),
	}
}

/// Parses `BlockString` token.
///
/// BlockString ::
///   """ `BlockStringCharacter` *list*,*opt* """
///
/// Spec: https://spec.graphql.org/draft/#BlockString
pub(crate) fn try_parse_block_string(lexer: &mut Lexer) -> TokenResult {
	let location = lexer.loc();

	// Skip leading `"""`
	lexer.eat_by(3);

	let mut raw_value = String::new();

	while !lexer.try_is(DOUBLE_QUOTE)? || !lexer.lookahead(DOUBLE_QUOTE, 2) {
		raw_value.push_str(parse_block_string_character(lexer).as_str());
	}

	// Skip trailing `"""`
	lexer.eat_by(3);

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
fn parse_block_string_character(lexer: &mut Lexer) -> String {
	match lexer.peek() {
		// BlockStringCharacter :: \"""
		BACKSLASH if lexer.lookahead(DOUBLE_QUOTE, 3) => {
			// Skip `\"""`
			lexer.eat_by(4);

			static_semantic_escaped_block_string_terminator()
		}
		CARRIAGE_RETURN if lexer.is_by(NEW_LINE, 1) => {
			// Skip `<CR><LF>`
			lexer.eat();
			lexer.eat_line();

			format!("{CARRIAGE_RETURN}{NEW_LINE}")
		}
		line_terminator @ (line_terminator!()) => {
			lexer.eat_line();
			line_terminator.into()
		}
		// BlockStringCharacter :: `SourceCharacter` but not """ or \"""
		char => {
			lexer.eat();

			static_semantic_block_string_character(char).into()
		}
	}
}
