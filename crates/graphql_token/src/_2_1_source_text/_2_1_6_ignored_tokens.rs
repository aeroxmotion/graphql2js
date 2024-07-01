/// UnicodeBOM ::
///   Byte Order Mark (U+FEFF)
///
/// Spec: https://spec.graphql.org/draft/#UnicodeBOM
pub const UNICODE_BOM: char = '\u{FEFF}';

/// Ignored ::
///   `UnicodeBOM`
///   `WhiteSpace`
///   `LineTerminator`
///   `Comment`
///   `Comma`
///
/// Spec: https://spec.graphql.org/draft/#Ignored
#[macro_export]
macro_rules! Ignored {
	() => {
		UNICODE_BOM | WhiteSpace!() | LineTerminator!() | COMMENT | COMMA
	};
}
