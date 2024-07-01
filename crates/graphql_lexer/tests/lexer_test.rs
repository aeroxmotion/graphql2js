#![feature(assert_matches)]
use std::{assert_matches::assert_matches, fs};

use graphql_lexer::Lexer;
use graphql_token::punctuator::*;
use graphql_token::token::Token;

macro_rules! assert_token {
	($token:expr, punctuator $punctuator:expr) => {
		assert_matches!($token, Token::Punctuator(punctuator, _) if *punctuator == $punctuator);
	};
	($token:expr, name $name:expr) => {
		assert_matches!($token, Token::Name(name, _) if name == $name);
	};
	($token:expr, string $string:expr) => {
		assert_matches!($token, Token::String(string, _) if string == $string);
	};
	($token:expr, block $string:expr) => {
		assert_matches!($token, Token::BlockString(string, _) if string == $string);
	};
}

#[test]
fn it_should_tokenize_correctly() {
	let tokens = Lexer::tokenize(fs::read_to_string("../../fixtures/simple.gql").unwrap()).unwrap();
	let mut cursor = 0;
	let mut next_token = || {
		let current = &tokens[cursor];

		cursor += 1;
		current
	};

	assert_matches!(next_token(), Token::SOF);

	// Directive definition
	assert_token!(next_token(), name "directive");
	assert_token!(next_token(), punctuator AT_SIGN_PUNCTUATOR);
	assert_token!(next_token(), name "MyDirective");
	assert_token!(next_token(), punctuator OPEN_PARENTHESIS_PUNCTUATOR);
	assert_token!(next_token(), name "arg");
	assert_token!(next_token(), punctuator COLON_PUNCTUATOR);
	assert_token!(next_token(), name "String");
	assert_token!(next_token(), punctuator CLOSE_PARENTHESIS_PUNCTUATOR);
	assert_token!(next_token(), name "on");
	assert_token!(next_token(), name "QUERY");

	// Union definition
	assert_token!(next_token(), name "union");
	assert_token!(next_token(), name "MyUnion");
	assert_token!(next_token(), punctuator EQUALS_PUNCTUATOR);
	assert_token!(next_token(), name "SomeValue");
	assert_token!(next_token(), punctuator VERTICAL_BAR_PUNCTUATOR);
	assert_token!(next_token(), name "AnotherValue");

	// Interface definition (1)
	assert_token!(next_token(), name "interface");
	assert_token!(next_token(), name "MyInterface");
	assert_token!(next_token(), punctuator OPENING_BRACE_PUNCTUATOR);
	assert_token!(next_token(), name "another_field");
	assert_token!(next_token(), punctuator COLON_PUNCTUATOR);
	assert_token!(next_token(), name "Float");
	assert_token!(next_token(), punctuator CLOSING_BRACE_PUNCTUATOR);

	// Interface definition (2)
	assert_token!(next_token(), name "interface");
	assert_token!(next_token(), name "MyOtherInterface");
	assert_token!(next_token(), punctuator OPENING_BRACE_PUNCTUATOR);
	assert_token!(next_token(), name "other_field");
	assert_token!(next_token(), punctuator COLON_PUNCTUATOR);
	assert_token!(next_token(), punctuator OPENING_BRACKET_PUNCTUATOR);
	assert_token!(next_token(), name "String");
	assert_token!(next_token(), punctuator CLOSING_BRACKET_PUNCTUATOR);
	assert_token!(next_token(), punctuator CLOSING_BRACE_PUNCTUATOR);

	// Type definition
	assert_token!(next_token(), block "Example block\nString");
	assert_token!(next_token(), name "type");
	assert_token!(next_token(), name "MyType");
	assert_token!(next_token(), name "implements");
	assert_token!(next_token(), name "MyInterface");
	assert_token!(next_token(), punctuator AMPERSAND_PUNCTUATOR);
	assert_token!(next_token(), name "MyOtherInterface");
	assert_token!(next_token(), punctuator OPENING_BRACE_PUNCTUATOR);
	assert_token!(next_token(), string "A field description");
	assert_token!(next_token(), name "my_field");
	assert_token!(next_token(), punctuator COLON_PUNCTUATOR);
	assert_token!(next_token(), name "Int");
	assert_token!(next_token(), punctuator EXCLAMATION_MARK_PUNCTUATOR);
	assert_token!(next_token(), punctuator CLOSING_BRACE_PUNCTUATOR);

	assert_matches!(next_token(), Token::EOF);
}
