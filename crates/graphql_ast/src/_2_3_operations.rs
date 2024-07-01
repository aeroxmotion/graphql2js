use crate::_2_10_variables::AstVariablesDefinition;
use crate::_2_4_selection_sets::AstSelectionSet;
use crate::location::AstLocation;

/// OperationDefinition :
///   `OperationType` `Name`? `VariablesDefinition`? `Directives`? `SelectionSet`
///   `SelectionSet`
///
/// Spec: https://spec.graphql.org/draft/#OperationDefinition
#[derive(Clone, Debug)]
pub struct AstOperationDefinition {
	/// `OperationType`
	pub operation_type: Option<AstOperationType>,

	/// `Name`?
	pub name: Option<AstName>,

	/// `VariablesDefinition`?
	pub variables: Option<AstVariablesDefinition>,

	/// `Directives`?
	pub directives: Option<AstDirectives>,

	/// `SelectionSet`
	pub selection_set: Option<AstSelectionSet>,

	/// Node's location
	pub location: AstLocation,
}

/// OperationType : one of
///   query mutation subscription
///
/// Spec: https://spec.graphql.org/draft/#OperationType
pub type AstOperationType = AstName;
