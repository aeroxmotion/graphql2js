use graphql_token::{
	punctuator::{EXCLAMATION_MARK_PUNCTUATOR, OPENING_BRACE_PUNCTUATOR, VERTICAL_BAR_PUNCTUATOR},
	token::Token,
};

use crate::{
	ast::{
		location::AstLocation,
		type_::reference::{AstListType, AstNamedType, AstType},
	},
	error::ParserError,
	Parser,
};

/// Parses `Type`
pub fn parse_type(parser: &mut Parser) -> Result<AstType, ParserError> {
	match parser.token() {
		Token::Name(name, _) => parse_named_type(parser, name.clone()),
		Token::Punctuator(punctuator, _) if punctuator == &OPENING_BRACE_PUNCTUATOR => {
			parse_list_type(parser)
		}
		token => Err(ParserError::UnexpectedToken(token.clone())),
	}
}

/// Parses `NamedType` / `NamedType !`
pub fn parse_named_type(parser: &mut Parser, name: String) -> Result<AstType, ParserError> {
	// Skip `name`
	parser.next();

	let named_type = AstNamedType {
		name,
		location: AstLocation {},
	};

	Ok(if parser.skip_if_punctuator(EXCLAMATION_MARK_PUNCTUATOR) {
		AstType::NonNullNamed(named_type)
	} else {
		AstType::Named(named_type)
	})
}

/// Parses `ListType` / `ListType !`
pub fn parse_list_type(parser: &mut Parser) -> Result<AstType, ParserError> {
	// Skip `[`
	parser.next();

	let list_type = Box::new(AstListType {
		type_: parse_type(parser)?,
		location: AstLocation {},
	});

	Ok(if parser.skip_if_punctuator(VERTICAL_BAR_PUNCTUATOR) {
		AstType::NonNullList(list_type)
	} else {
		AstType::List(list_type)
	})
}
