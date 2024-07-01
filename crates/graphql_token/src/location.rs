#[derive(Clone, Debug)]
pub struct Location {
	/// Token index location
	pub index: usize,

	/// Token line location
	pub line: usize,

	/// Token column location
	pub column: usize,
}

impl Location {
	pub fn new(index: usize, line: usize, column: usize) -> Self {
		Self {
			index,
			line,
			column,
		}
	}
}
