use graphql_token::location::Location;

#[derive(Clone, Debug)]
pub struct AstLocation {
	/// Node's start location
	start: Location,

	/// Node's end location
	end: Location,
}
