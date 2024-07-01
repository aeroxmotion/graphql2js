use graphql_token::token::Token;

#[derive(Clone, Debug)]
pub enum ParserError {
	UnexpectedToken(Token),
}
