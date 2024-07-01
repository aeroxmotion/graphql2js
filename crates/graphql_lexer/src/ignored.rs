/// UnicodeBOM ::
///   Byte Order Mark (U+FEFF)
///
/// Spec: https://spec.graphql.org/draft/#UnicodeBOM
pub(crate) const UNICODE_BOM: char = '\u{FEFF}';

/// WhiteSpace ::
///   Horizontal Tab (U+0009)
///   Space (U+0020)
///
/// Spec: https://spec.graphql.org/draft/#WhiteSpace
macro_rules! white_space {
	() => {
		'\u{9}' | '\u{20}'
	};
}
pub(crate) use white_space;

/// New Line ::
///   U+000A
///
/// https://spec.graphql.org/draft/#LineTerminator
pub(crate) const NEW_LINE: char = '\u{A}';

/// New Line ::
///   U+000D
///
/// https://spec.graphql.org/draft/#LineTerminator
pub(crate) const CARRIAGE_RETURN: char = '\u{D}';

/// LineTerminator ::
///   `New Line`
///   `Carriage Return` [lookahead != New Line\]
///   `Carriage Return` `New Line`
///
/// Spec: https://spec.graphql.org/draft/#LineTerminator
macro_rules! line_terminator {
	() => {
		NEW_LINE | CARRIAGE_RETURN
	};
}
pub(crate) use line_terminator;

/// Comma ::
///   ,
///
/// Spec: https://spec.graphql.org/draft/#Comma
pub(crate) const COMMA: char = ',';

/// Spec: https://spec.graphql.org/draft/#Comment
pub(crate) const NUMBER_SIGN: char = '#';
