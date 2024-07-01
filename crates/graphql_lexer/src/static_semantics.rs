use std::assert_matches::assert_matches;
use std::collections::VecDeque;

use crate::{white_space, BACKSLASH, DOUBLE_QUOTE, NEW_LINE};

/// StringValue :: ""
///
/// Spec: https://spec.graphql.org/draft/#sec-String-Value.Static-Semantics
pub(crate) fn static_semantic_string_value_empty_sequence() -> String {
	// 1. Return an empty sequence.
	"".into()
}

/// StringCharacter :: `SourceCharacter` but not " or \ or `LineTerminator`
///
/// Spec: https://spec.graphql.org/draft/#sec-String-Value.Static-Semantics
pub(crate) fn static_semantic_string_character(source_character: char) -> char {
	// 1. Return the `Unicode scalar value` `SourceCharacter`.
	source_character
}

/// StringValue :: " `StringCharacter` *list* "
///
/// Spec: https://spec.graphql.org/draft/#sec-String-Value.Static-Semantics
pub(crate) fn static_semantic_string_character_list(string_characters: String) -> String {
	// 1. Return the `Unicode text` by concatenating the evaluation of all `StringCharacter`.
	string_characters
}

/// StringCharacter ::
///   \u `HexDigit` `HexDigit` `HexDigit` `HexDigit` \u `HexDigit` `HexDigit` `HexDigit` `HexDigit`
///
/// Spec (1): https://spec.graphql.org/draft/#sec-String-Value.Static-Semantics
/// Spec (2): https://www.unicode.org/glossary/#surrogate_pair
pub(crate) fn static_semantic_possible_unicode_surrogate_pair(
	first_hex_digit_sequence: String,
	second_hex_digit_sequence: String,
) -> String {
	// 1. Let leadingValue be the hexadecimal value represented by the first sequence of HexDigit.
	let leading_value = to_hex_value(&first_hex_digit_sequence);

	// 2. Let trailingValue be the hexadecimal value represented by the second sequence of HexDigit.
	let trailing_value = to_hex_value(&second_hex_digit_sequence);

	// 3. If leadingValue is ≥ 0xD800 and ≤ 0xDBFF (a Leading Surrogate):
	if matches!(leading_value, 0xD800..=0xDBFF) {
		// a. Assert trailingValue is ≥ 0xDC00 and ≤ 0xDFFF (a Trailing Surrogate).
		assert_matches!(
			trailing_value,
			0xDC00..=0xDFFF,
			"Invalid low-surrogate \\u{second_hex_digit_sequence}"
		);

		// b. Return (leadingValue - 0xD800) × 0x400 + (trailingValue - 0xDC00) + 0x10000.
		into_unicode_scalar_value(
			(leading_value - 0xD800) * 0x400 + (trailing_value - 0xDC00) + 0x10000,
		)
		.into()

	// 4. Otherwise:
	} else {
		// a. Assert leadingValue is within the `Unicode scalar value` range.
		assert_within_unicode_scalar_value_range(leading_value, &first_hex_digit_sequence);

		// b. Assert trailingValue is within the `Unicode scalar value` range.
		assert_within_unicode_scalar_value_range(trailing_value, &second_hex_digit_sequence);

		// c. Return the sequence of the `Unicode scalar value` leadingValue followed by the `Unicode scalar value` trailingValue.
		format!(
			"{}{}",
			into_unicode_scalar_value(leading_value),
			into_unicode_scalar_value(trailing_value)
		)
	}
}

/// StringCharacter :: \u `EscapedUnicode`
///
/// Spec: https://spec.graphql.org/draft/#sec-String-Value.Static-Semantics
pub(crate) fn static_semantic_escaped_unicode(hex_digit_sequence: String) -> char {
	// 1. Let value be the hexadecimal value represented by the sequence of `HexDigit` within `EscapedUnicode`.
	let value = to_hex_value(&hex_digit_sequence);

	// 2. Assert value is within the `Unicode scalar value` range (≥ 0x0000 and ≤ 0xD7FF or ≥ 0xE000 and ≤ 0x10FFFF).
	assert_within_unicode_scalar_value_range(value, &hex_digit_sequence);

	// 3. Return the `Unicode scalar value` value.
	into_unicode_scalar_value(value)
}

/// StringCharacter :: \ `EscapedCharacter`
///
/// Spec: https://spec.graphql.org/draft/#sec-String-Value.Static-Semantics
pub(crate) fn static_semantic_escaped_character(escaped_character: char) -> Option<char> {
	// 1. Return the _Unicode scalar value_ represented by `EscapedCharacter` according to the table below.
	//
	// -----------------------------------
	// | Escaped Charater | Scalar Value |
	// -----------------------------------
	// | "                | U+0022       |
	// | \                | U+005C       |
	// | /                | U+002F       |
	// | b                | U+0008       |
	// | f                | U+000C       |
	// | n                | U+000A       |
	// | r                | U+000D       |
	// | t                | U+0009       |
	// -----------------------------------
	match escaped_character {
		DOUBLE_QUOTE => Some('\u{22}'),
		BACKSLASH => Some('\u{5C}'),
		'/' => Some('\u{2F}'),
		'b' => Some('\u{8}'),
		'f' => Some('\u{C}'),
		'n' => Some('\u{A}'),
		'r' => Some('\u{D}'),
		't' => Some('\u{9}'),
		_ => None,
	}
}

/// BlockString :: """ `BlockStringCharacter` *list*,*opt* """
///
/// Spec: https://spec.graphql.org/draft/#sec-String-Value.Static-Semantics
pub(crate) fn static_semantic_block_string_character_list(
	block_string_characters: String,
) -> String {
	// 1. Let rawValue be the Unicode text by concatenating the evaluation of
	// all BlockStringCharacter (which may be an empty sequence).
	let raw_value = block_string_characters.clone();

	// 2. Return the result of BlockStringValue(rawValue).
	block_string_value(raw_value)
}

/// BlockStringCharacter :: \"""
///
/// Spec: https://spec.graphql.org/draft/#sec-String-Value.Static-Semantics
pub(crate) fn static_semantic_escaped_block_string_terminator() -> String {
	// 1. Return the character sequence """.
	r#"""""#.into()
}

/// BlockStringCharacter :: `SourceCharacter` but not """ or \"""
///
/// Spec: https://spec.graphql.org/draft/#sec-String-Value.Static-Semantics
pub(crate) fn static_semantic_block_string_character(source_character: char) -> char {
	// 1. Return the `Unicode scalar value` `SourceCharacter`.
	source_character
}

/// BlockStringValue(rawValue)
///
/// Spec: https://spec.graphql.org/draft/#BlockStringValue()
fn block_string_value(raw_value: String) -> String {
	// 1. Let lines be the result of splitting rawValue by LineTerminator.
	let mut lines: VecDeque<String> = raw_value.lines().map(|str| str.to_string()).collect();

	// 2. Let commonIndent be null.
	let mut common_indent: Option<usize> = None;

	// 3. For each line in lines:
	for (i, line) in lines.iter().enumerate() {
		// a. If line is the first item in lines, continue to the next line.
		if i == 0 {
			continue;
		}

		let chars_ = line.chars();

		// b. Let length be the number of characters in line.
		let length = chars_.clone().count();

		// c. Let indent be the number of leading consecutive WhiteSpace characters in line.
		let mut indent = 0;

		for char in chars_ {
			if !matches!(char, white_space!()) {
				break;
			}

			indent += 1;
		}

		// d. If indent is less than length:
		if indent < length {
			// i. If commonIndent is null or indent is less than commonIndent:
			if common_indent.is_none() || indent < common_indent.unwrap() {
				// 1. Let commonIndent be indent.
				common_indent = Some(indent);
			}
		}
	}

	// 4. If commonIndent is not null:
	if let Some(common_indent_) = common_indent {
		// a. For each line in lines:
		for (i, line) in lines.iter_mut().enumerate() {
			// i. If line is the first item in lines, continue to the next line.
			if i == 0 {
				continue;
			}

			if !line.is_empty() {
				// ii. Remove commonIndent characters from the beginning of line.
				*line = (&line[common_indent_..]).to_string();
			}
		}
	}

	// 5. While the first item line in lines contains only `WhiteSpace`:
	while !lines.is_empty() && lines[0].chars().all(|char| matches!(char, white_space!())) {
		// a. Remove the first item from lines.
		lines.pop_front();
	}

	// 6. While the last item line in lines contains only `WhiteSpace`:
	while !lines.is_empty()
		&& lines[lines.len() - 1]
			.chars()
			.all(|char| matches!(char, white_space!()))
	{
		// a. Remove the last item from lines.
		lines.pop_back();
	}

	// 7. Let formatted be the empty character sequence.
	let mut formatted = String::new();

	// 8. For each line in lines:
	for (i, line) in lines.iter().enumerate() {
		// a. If line is the first item in lines:
		if i == 0 {
			// i. Append formatted with line.
			formatted.push_str(line);

		// b. Otherwise:
		} else {
			// i. Append formatted with a line feed character (U+000A).
			formatted.push(NEW_LINE);

			// ii. Append formatted with line.
			formatted.push_str(line);
		}
	}

	// 9. Return formatted.
	formatted
}

/// Returns the hexadecimal value from the given `HexDigit` sequence.
fn to_hex_value(hex_digit_sequence: &str) -> u32 {
	u32::from_str_radix(hex_digit_sequence, 16).unwrap()
}

/// Returns the `Unicode scalar value` from the given hexadecimal value.
fn into_unicode_scalar_value(hex_value: u32) -> char {
	char::from_u32(hex_value).unwrap()
}

/// Spec: https://www.unicode.org/glossary/#unicode_scalar_value
fn assert_within_unicode_scalar_value_range(value: u32, debug_value: &str) {
	assert_matches!(value, 0..=0xD7FF | 0xE000..=0x10FFFF, "Invalid unicode `\\u{debug_value}`")
}
