use crate::{
	ast::{
		location::AstLocation,
		value::{
			boolean::AstBooleanValue,
			enum_::AstEnumValue,
			float::AstFloatValue,
			int::AstIntValue,
			list::AstListValue,
			null::AstNullValue,
			object::{AstObjectField, AstObjectValue},
			string::AstStringValue,
			AstValue,
		},
	},
	error::ParserError,
	Parser,
};
use graphql_token::{
	punctuator::{
		CLOSING_BRACE_PUNCTUATOR, CLOSING_BRACKET_PUNCTUATOR, COLON_PUNCTUATOR,
		OPENING_BRACE_PUNCTUATOR, OPENING_BRACKET_PUNCTUATOR,
	},
	token::Token,
};

use super::name::parse_name;

/// Parses `Value`
pub fn parse_value(parser: &mut Parser) -> Result<AstValue, ParserError> {
	let token = parser.token();

	let value = match token {
		Token::Int(value, _) => Ok(AstValue::Int(AstIntValue {
			raw: value.clone(),
			value: value.parse::<isize>().unwrap(), // TODO: Handle parsing error
			location: AstLocation {},
		})),
		Token::Float(value, _) => Ok(AstValue::Float(AstFloatValue {
			raw: value.clone(),
			value: value.parse::<f64>().unwrap(), // TODO: Handle parsing error
			location: AstLocation {},
		})),
		Token::String(value, _) => Ok(AstValue::String(AstStringValue {
			value: value.clone(),
			block: false,
			location: AstLocation {},
		})),
		Token::BlockString(value, _) => Ok(AstValue::String(AstStringValue {
			value: value.clone(),
			block: true,
			location: AstLocation {},
		})),
		Token::Name(name, _) => match name.as_str() {
			"true" => Ok(AstValue::Boolean(AstBooleanValue {
				value: true,
				location: AstLocation {},
			})),
			"false" => Ok(AstValue::Boolean(AstBooleanValue {
				value: false,
				location: AstLocation {},
			})),
			"null" => Ok(AstValue::Null(AstNullValue {
				location: AstLocation {},
			})),
			// Enum
			_ => Ok(AstValue::Enum(AstEnumValue {
				name: name.clone(),
				location: AstLocation {},
			})),
		},
		Token::Punctuator(punctuator, _) => match *punctuator {
			OPENING_BRACKET_PUNCTUATOR => Ok(AstValue::List(Box::new(parse_list_value(parser)?))),
			OPENING_BRACE_PUNCTUATOR => Ok(AstValue::Object(Box::new(parse_object_value(parser)?))),
			_ => Err(ParserError::UnexpectedToken(token.clone())),
		},
		_ => Err(ParserError::UnexpectedToken(token.clone())),
	}?;

	// Skip `Value`
	parser.next();

	Ok(value)
}

/// Parses for `ListValue`
pub fn parse_list_value(parser: &mut Parser) -> Result<AstListValue, ParserError> {
	// Skip open bracket `[`
	parser.next();

	let mut values = vec![];

	while !matches!(parser.token(), Token::Punctuator(punctuator, _) if punctuator == &CLOSING_BRACKET_PUNCTUATOR)
	{
		values.push(parse_value(parser)?);
	}

	// Skip close bracket `]`
	parser.next();

	Ok(AstListValue {
		values,
		location: AstLocation {},
	})
}

/// Parses for `ObjectValue`
pub fn parse_object_value(parser: &mut Parser) -> Result<AstObjectValue, ParserError> {
	// Skip open brace `{`
	parser.next();

	let mut fields = vec![];

	while !matches!(parser.token(), Token::Punctuator(punctuator, _) if punctuator == &CLOSING_BRACE_PUNCTUATOR)
	{
		fields.push(parse_object_field(parser)?);
	}

	// Skip close brace `}`
	parser.next();

	Ok(AstObjectValue {
		fields,
		location: AstLocation {},
	})
}

/// Parses for `ObjectField`
pub fn parse_object_field(parser: &mut Parser) -> Result<AstObjectField, ParserError> {
	let name = parse_name(parser)?;

	// Skip `:`
	parser.expect_punctuator(COLON_PUNCTUATOR)?;

	Ok(AstObjectField {
		name,
		value: parse_value(parser)?,
		location: AstLocation {},
	})
}
