/// Letter :: one of
///   A...Z a...z
///
/// Spec: https://spec.graphql.org/draft/#Letter
#[macro_export]
macro_rules! Letter {
	() => {
		'A'..='Z' | 'a'..='z'
	};
}

/// Digit :: one of
///   0...9
///
/// Spec: https://spec.graphql.org/draft/#Digit
#[macro_export]
macro_rules! Digit {
	() => {
		'0'..='9'
	};
}

/// NameStart ::
///   `Letter`
///   _
///
/// Spec: https://spec.graphql.org/draft/#NameStart
#[macro_export]
macro_rules! NameStart {
	() => {
		Letter!() | '_'
	};
}

/// NameContinue ::
///   `Letter`
///   `Digit`
///   _
///
/// Spec: https://spec.graphql.org/draft/#NameContinue
#[macro_export]
macro_rules! NameContinue {
	() => {
		Letter!() | Digit!() | '_'
	};
}
