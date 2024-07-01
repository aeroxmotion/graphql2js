use crate::_2_12_directives::AstDirectives;
use crate::_2_1_8_names::AstName;
use crate::_2_4_selection_sets::AstSelectionSet;
use crate::_2_6_arguments::AstArguments;
use crate::_2_7_field_alias::AstAlias;
use crate::location::AstLocation;

/// Field :
///   `Alias`? `Name` `Arguments`? `Directives`? `SelectionSet`?
///
/// Spec: https://spec.graphql.org/draft/#Field
#[derive(Clone, Debug)]
pub struct AstField {
	/// `Alias`?
	pub alias: Option<AstAlias>,

	/// `Name`
	pub name: AstName,

	/// `Arguments`?
	pub arguments: Option<AstArguments>,

	/// `Directives`?
	pub directives: Option<AstDirectives>,

	/// `SelectionSet`?
	pub selection_set: Option<Box<AstSelectionSet>>,

	/// Node's location
	pub location: AstLocation,
}
