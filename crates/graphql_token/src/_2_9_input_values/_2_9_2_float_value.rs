/// ExponentIndicator :: one of
///   e E
///
/// Spec: https://spec.graphql.org/draft/#ExponentIndicator
#[macro_export]
macro_rules! ExponentIndicator {
	() => {
		'e' | 'E'
	};
}

/// Sign :: one of
///   + -
///
/// Spec: https://spec.graphql.org/draft/#Sign
#[macro_export]
macro_rules! Sign {
	() => {
		'+' | '-'
	};
}
