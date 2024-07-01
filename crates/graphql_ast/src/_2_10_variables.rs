use crate::_2_11_type_references::AstType;
use crate::_2_12_directives::AstDirectivesConst;
use crate::_2_1_8_names::AstName;
use crate::_2_9_input_values::AstValueConst;
use crate::location::AstLocation;

/// Variable :
///   $ `Name`
///
/// Spec: https://spec.graphql.org/draft/#Variable
pub type AstVariable = AstName;

/// VariablesDefinition :
///   ( `VariableDefinition`+ )
///
/// Spec: https://spec.graphql.org/draft/#VariablesDefinition
#[derive(Clone, Debug)]
pub struct AstVariablesDefinition {
	/// `VariableDefinition`+
	pub definitions: Vec<AstVariableDefinition>,

	/// Node's location
	pub location: AstLocation,
}

/// VariableDefinition :
///   `Variable` : `Type` `DefaultValue`? `Directives[Const]`?
///
/// Spec: https://spec.graphql.org/draft/#VariableDefinition
#[derive(Clone, Debug)]
pub struct AstVariableDefinition {
	/// `Variable`
	pub variable: AstVariable,

	/// `Type`
	pub type_: AstType,

	/// `DefaultValue`?
	pub default_value: Option<AstDefaultValue>,

	/// `Directives[Const]`?
	pub directives: Option<AstDirectivesConst>,

	/// Node's location
	pub location: AstLocation,
}

/// DefaultValue :
///   = `Value[Const]`
///
/// Spec: https://spec.graphql.org/draft/#DefaultValue
pub type AstDefaultValue = AstValueConst;
