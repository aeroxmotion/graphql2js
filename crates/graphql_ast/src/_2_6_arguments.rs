use super::location::AstLocation;

/// Arguments[Const] :
///   ( `Argument[?Const]`+ )
///
/// Spec: https://spec.graphql.org/draft/#Arguments
#[derive(Clone, Debug)]
pub struct AstArguments {
	/// `Argument[?Const]`+
	pub arguments: Vec<AstArgument>,

	/// Node's location
	pub location: AstLocation,
}

/// Argument[Const] :
///   `Name` : `Value[?Const]`
///
/// Spec: https://spec.graphql.org/draft/#Argument
#[derive(Clone, Debug)]
pub struct AstArgument {
	/// `Name`
	pub name: AstName,

	/// `Value[?Const]`
	pub value: AstValue,

	/// Node's location
	pub location: AstLocation,
}

// TODO: Do `[Const]`
