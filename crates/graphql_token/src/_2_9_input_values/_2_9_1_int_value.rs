/// NegativeSign ::
///   -
///
/// Spec: https://spec.graphql.org/draft/#NegativeSign
pub const NEGATIVE_SIGN: char = '-';

/// NonZeroDigit ::
///   1...9
///
/// Spec: https://spec.graphql.org/draft/#NonZeroDigit
#[macro_export]
macro_rules! NonZeroDigit {
	() => {
		'1'..='9'
	};
}
