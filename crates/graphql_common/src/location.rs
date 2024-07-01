/// Token's location
#[derive(Clone, Debug)]
pub struct TokenLocation {
	/// Token index location
	pub index: usize,

	/// Token line location
	pub line: usize,

	/// Token column location
	pub column: usize,
}

impl TokenLocation {
	pub fn new(index: usize, line: usize, column: usize) -> Self {
		Self {
			index,
			line,
			column,
		}
	}
}

/// Node's location
#[derive(Clone, Debug)]
pub struct AstLocation {
	/// Node's start location
	pub start: TokenLocation,

	/// Node's end location
	pub end: TokenLocation,
}
