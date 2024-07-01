use super::AstValue;
use super::AstValueConst;
use crate::location::AstLocation;

/// ObjectValue :
///   { }
///   {} `ObjectField`+ }
///
/// Spec: https://spec.graphql.org/draft/#ObjectValue
#[derive(Clone, Debug)]
pub struct AstObjectValue {
	/// `ObjectField`+
	pub values: Vec<AstObjectField>,

	/// Node's location
	pub location: AstLocation,
}

/// ObjectValue[Const] :
///   { }
///   { `ObjectField[Const]`+ }
///
/// Spec: https://spec.graphql.org/draft/#ObjectValue
#[derive(Clone, Debug)]
pub struct AstObjectValueConst {
	/// `Value`+
	pub values: Vec<AstObjectFieldConst>,

	/// Node's location
	pub location: AstLocation,
}

/// ObjectField :
///   `Name` : `Value`
///
/// Spec: https://spec.graphql.org/draft/#ObjectValue
#[derive(Clone, Debug)]
pub struct AstObjectField {
	/// `Name`
	pub name: AstName,

	/// `Value`
	pub value: AstValue,

	/// Node's location
	pub location: AstLocation,
}

/// ObjectField[Const] :
///   `Name` : `Value[Const]`
///
/// Spec: https://spec.graphql.org/draft/#ObjectValue
#[derive(Clone, Debug)]
pub struct AstObjectFieldConst {
	/// `Name`
	pub name: AstName,

	/// `Value`
	pub value: AstValueConst,

	/// Node's location
	pub location: AstLocation,
}
