use crate::_2_11_type_references::AstNamedType;

/// TypeCondition :
///   on `NamedType`
///
/// Spec: https://spec.graphql.org/draft/#TypeCondition
pub type AstTypeCondition = AstNamedType;
