use crate::_2_12_directives::AstDirectives;
use crate::_2_4_selection_sets::AstSelectionSet;
use crate::_2_8_1_type_conditions::AstTypeCondition;
use crate::location::AstLocation;

/// FragmentSpread :
///   ... `FragmentName` `Directives`?
///
/// Spec: https://spec.graphql.org/draft/#FragmentSpread
#[derive(Clone, Debug)]
pub struct AstFragmentSpread {
	/// `FragmentName`
	pub name: AstFragmentName,

	/// `Directives`?
	pub directives: Option<AstDirectives>,

	/// Node's location
	pub location: AstLocation,
}

/// FragmentDefinition :
///   fragment `FragmentName` `TypeCondition` `Directives`? `SelectionSet`
///
/// Spec: https://spec.graphql.org/draft/#FragmentDefinition
#[derive(Clone, Debug)]
pub struct AstFragmentDefinition {
	/// `FragmentName`
	pub name: AstFragmentName,

	/// `TypeCondition`
	pub type_condition: AstTypeCondition,

	/// `Directives`?
	pub directives: Option<AstDirectives>,

	/// `SelectionSet`
	pub selection_set: AstSelectionSet,

	/// Node's location
	pub location: AstLocation,
}

/// FragmentName :
///   `Name` but not _on_
///
/// Spec: https://spec.graphql.org/draft/#FragmentName
pub type AstFragmentName = AstName;
