/// LineTerminator ::
///   `New Line`
///   `Carriage Return` [lookahead != `New Line`\]
///   `Carriage Return` `New Line`
///
/// Spec: https://spec.graphql.org/draft/#LineTerminator
#[macro_export]
macro_rules! LineTerminator {
	() => {
		NEW_LINE | CARRIAGE_RETURN
	};
}

/// LineTerminator :: New Line (U+000A)
///
/// Spec: https://spec.graphql.org/draft/#LineTerminator
pub const NEW_LINE: char = '\u{A}';

/// LineTerminator :: Carriage Return (U+000D)
///
/// Spec: https://spec.graphql.org/draft/#LineTerminator
pub const CARRIAGE_RETURN: char = '\u{D}';
