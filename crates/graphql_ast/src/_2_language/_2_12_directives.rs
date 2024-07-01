use super::_2_1_8_names::AstName;
use super::_2_6_arguments::AstArguments;
use super::_2_6_arguments::AstArgumentsConst;
use crate::location::AstLocation;

/// Directives :
///   `Directive`+
///
/// Spec: https://spec.graphql.org/draft/#Directives
#[derive(Clone, Debug)]
pub struct AstDirectives {
	/// `Directive`+
	pub directives: Vec<AstDirective>,

	/// Node's location
	pub location: AstLocation,
}

/// Directives[Const] :
///   `Directive[Const]`+
///
/// Spec: https://spec.graphql.org/draft/#Directives
#[derive(Clone, Debug)]
pub struct AstDirectivesConst {
	/// `Directive[Const]`+
	pub directives: Vec<AstDirectiveConst>,

	/// Node's location
	pub location: AstLocation,
}

/// Directive :
///   @ `Name` `Arguments`?
///
/// Spec: https://spec.graphql.org/draft/#Directive
#[derive(Clone, Debug)]
pub struct AstDirective {
	/// `Name`
	pub name: AstName,

	/// `Arguments`?
	pub arguments: Option<AstArguments>,

	/// Node's location
	pub location: AstLocation,
}

/// Directive[Const] :
///   @ `Name` `Arguments[Const]`?
///
/// Spec: https://spec.graphql.org/draft/#Directive
#[derive(Clone, Debug)]
pub struct AstDirectiveConst {
	/// `Name`
	pub name: AstName,

	/// `Arguments`?
	pub arguments: Option<AstArgumentsConst>,

	/// Node's location
	pub location: AstLocation,
}
