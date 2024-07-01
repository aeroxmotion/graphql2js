use super::_2_8_1_type_conditions::AstTypeCondition;
use crate::_2_language::_2_12_directives::AstDirectives;
use crate::_2_language::_2_4_selection_sets::AstSelectionSet;
use crate::location::AstLocation;

/// InlineFragment :
///   ... `TypeCondition`? `Directives`? `SelectionSet`
///
/// Spec: https://spec.graphql.org/draft/#InlineFragment
#[derive(Clone, Debug)]
pub struct AstInlineFragment {
	/// `TypeCondition`?
	pub type_condition: Option<AstTypeCondition>,

	/// `Directives`?
	pub directives: Option<AstDirectives>,

	/// `SelectionSet`
	pub selection_set: AstSelectionSet,

	/// Node's location
	pub location: AstLocation,
}
