use crate::_2_language::_2_1_8_names::AstName;

/// EnumValue :
///   `Name` but not _true_ or _false_ or _null_
///
/// Spec: https://spec.graphql.org/draft/#EnumValue
pub type AstEnumValue = AstName;
