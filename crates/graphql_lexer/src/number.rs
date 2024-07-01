use graphql_token::location::Location;
use graphql_token::token::Token;

use super::digit;
use super::error::LexerError;
use super::match_punctuator;
use super::Lexer;
use super::TokenResult;

/// NegativeSign ::
///   -
///
/// Spec: https://spec.graphql.org/draft/#NegativeSign
pub(crate) const NEGATIVE_SIGN: char = '-';

/// NonZeroDigit ::
///   `Digit` but not 0
///
/// Spec: https://spec.graphql.org/draft/#NonZeroDigit
macro_rules! non_zero_digit {
	() => {
		'1'..='9'
	};
}

pub(crate) fn try_parse_unsigned_number(lexer: &mut Lexer) -> TokenResult {
	try_parse_number(lexer, "", lexer.loc())
}

pub(crate) fn try_parse_signed_number(lexer: &mut Lexer) -> TokenResult {
	let location = lexer.loc();

	// Skip `-` sign
	lexer.eat();

	try_parse_number(lexer, &NEGATIVE_SIGN.to_string(), location)
}

/// Parses `IntValue` / `FloatValue` token.
///
/// IntValue ::
///   `IntegerPart` [lookahead != {Digit, ., NameStart}\]
///
/// Spec: https://spec.graphql.org/draft/#IntValue
///
/// FloatValue ::
///   `IntegerPart` `FractionalPart` `ExponentPart` [lookahead != {Digit, ., NameStart}]
///   `IntegerPart` `FractionalPart` [lookahead != {Digit, ., NameStart}]
///   `IntegerPart` `ExponentPart` [lookahead != {Digit, ., NameStart}]
///
/// Spec: https://spec.graphql.org/draft/#FloatValue
fn try_parse_number(lexer: &mut Lexer, sign: &str, location: Location) -> TokenResult {
	let integer_part = try_parse_integer_part(lexer)?;
	let fractional_part = try_parse_fractional_part(lexer)?;
	let exponent_part = try_parse_exponent_part(lexer)?;

	let value = format!("{sign}{integer_part}{fractional_part}{exponent_part}");

	Ok(if fractional_part.len().max(exponent_part.len()) > 0 {
		Token::Float(value, location)
	} else {
		Token::Int(value, location)
	})
}

/// Parses `IntegerPart`.
///
/// IntegerPart ::
///   `NegativeSign` *opt* 0
///   `NegativeSign` *opt* `NonZeroDigit` `Digit`
///
/// Spec: https://spec.graphql.org/draft/#IntegerPart
fn try_parse_integer_part(lexer: &mut Lexer) -> Result<String, LexerError> {
	let mut part = String::new();

	match lexer.try_peek()? {
		'0' => part.push('0'),
		non_zero_digit!() => {
			while {
				part.push(lexer.peek());
				lexer.eat();

				match_punctuator!(lexer, digit!())
			} {}
		}
		char => return Err(LexerError::InvalidToken(char, lexer.loc())),
	}

	Ok(part)
}

macro_rules! try_parse_optional_part {
	($lexer:ident, $prefix:pat $(, $sign:pat)?) => {{
		let mut part = String::new();

		if match_punctuator!($lexer, $prefix) {
			// Skip `e`, `E` or  `.`
			$lexer.eat();

			match $lexer.try_peek()? {
				digit!() => part.push($lexer.peek()),
				$($sign => {
					part.push($lexer.peek());

					// Skip sign
					$lexer.eat();

					// After processing sign, we expect a digit
					match $lexer.try_peek()? {
						digit @ digit!() => part.push(digit),
						char => return Err(LexerError::InvalidToken(char, $lexer.loc())),
					}
				},)?
				char => return Err(LexerError::InvalidToken(char, $lexer.loc())),
			}

			// Skip first digit
			$lexer.eat();

			while match_punctuator!($lexer, digit!()) {
				part.push($lexer.peek());
				$lexer.eat();
			}
		}

		Ok(part)
	}};
}

/// Parses `FractionalPart`.
///
/// FractionalPart ::
///   . `Digit` *list*
///
/// Spec: https://spec.graphql.org/draft/#FractionalPart
fn try_parse_fractional_part(lexer: &mut Lexer) -> Result<String, LexerError> {
	try_parse_optional_part!(lexer, '.')
}

/// ExponentIndicator :: one of
///   e E
///
/// Spec: https://spec.graphql.org/draft/#ExponentIndicator
macro_rules! exponent_indicator {
	() => {
		'e' | 'E'
	};
}

/// Sign :: one of
///   - +
///
/// Spec: https://spec.graphql.org/draft/#Sign
macro_rules! sign {
	() => {
		'-' | '+'
	};
}

/// Parses `ExponentPart`
///
/// ExponentPart::
///   `ExponentIndicator` `Sign` *opt* `Digit` *list*
fn try_parse_exponent_part(lexer: &mut Lexer) -> Result<String, LexerError> {
	try_parse_optional_part!(lexer, exponent_indicator!(), sign!())
}
