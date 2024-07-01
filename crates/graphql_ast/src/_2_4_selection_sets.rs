use crate::_2_5_fields::AstField;
use crate::_2_8_2_inline_fragments::AstInlineFragment;
use crate::_2_8_fragments::AstFragmentSpread;
use crate::location::AstLocation;

/// SelectionSet :
///   { `Selection`+ }
///
/// Spec: https://spec.graphql.org/draft/#SelectionSet
#[derive(Clone, Debug)]
pub struct AstSelectionSet {
	/// `Selection`+
	pub selections: Vec<AstSelection>,

	/// Node's location
	pub location: AstLocation,
}

/// Selection :
///   `Field`
///   `FragmentSpread`
///   `InlineFragment`
#[derive(Clone, Debug)]
pub enum AstSelection {
	/// `Field`
	Field(AstField),

	/// `FragmentSpread`
	FragmentSpread(AstFragmentSpread),

	/// `InlineFragment`
	InlineFragment(AstInlineFragment),
}
