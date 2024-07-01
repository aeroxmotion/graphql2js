use graphql_common::location::AstLocation;

use super::_2_1_8_names::AstName;
use super::_2_9_input_values::AstValue;

/// Arguments :
///   ( `Argument`+ )
///
/// Spec: https://spec.graphql.org/draft/#Arguments
#[derive(Clone, Debug)]
pub struct AstArguments {
	/// `Argument`+
	pub arguments: Vec<AstArgument>,

	/// Node's location
	pub location: AstLocation,
}

/// Arguments[Const] :
///   ( `Argument[Const]`+ )
///
/// Spec: https://spec.graphql.org/draft/#Arguments
#[derive(Clone, Debug)]
pub struct AstArgumentsConst {
	/// `Argument`+
	pub arguments: Vec<AstArgumentConst>,

	/// Node's location
	pub location: AstLocation,
}

/// Argument :
///   `Name` : `Value`
///
/// Spec: https://spec.graphql.org/draft/#Argument
#[derive(Clone, Debug)]
pub struct AstArgument {
	/// `Name`
	pub name: AstName,

	/// `Value`
	pub value: AstValue,

	/// Node's location
	pub location: AstLocation,
}

/// Argument :
///   `Name` : `Value`
///
/// Spec: https://spec.graphql.org/draft/#Argument
#[derive(Clone, Debug)]
pub struct AstArgumentConst {
	/// `Name`
	pub name: AstName,

	/// `Value`
	pub value: AstValue,

	/// Node's location
	pub location: AstLocation,
}
