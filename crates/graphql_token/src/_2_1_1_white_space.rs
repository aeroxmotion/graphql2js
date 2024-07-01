/// WhiteSpace ::
///   `Horizontal Tab`
///   `Space`
///
/// Spec: https://spec.graphql.org/draft/#WhiteSpace
#[macro_export]
macro_rules! WhiteSpace {
	() => {
		HORIZONTAL_TAB | SPACE
	};
}

/// WhiteSpace :: Horizontal Tab (U+0009)
///
/// Spec: https://spec.graphql.org/draft/#WhiteSpace
pub const HORIZONTAL_TAB: char = '\u{9}';

/// WhiteSpace :: Space (U+0020)
///
/// Spec: https://spec.graphql.org/draft/#WhiteSpace
pub const SPACE: char = '\u{20}';
