use graphql_common::location::AstLocation;

use super::AstValue;
use super::AstValueConst;

/// List :
///  [ ]
///  [ `Value`+ ]
///
/// Spec: https://spec.graphql.org/draft/#ListValue
#[derive(Clone, Debug)]
pub struct AstListValue {
	/// `Value`+
	pub values: Vec<AstValue>,

	/// Node's location
	pub location: AstLocation,
}

/// List[Const] :
///  [ ]
///  [ `Value[Const]`+ ]
///
/// Spec: https://spec.graphql.org/draft/#ListValue
#[derive(Clone, Debug)]
pub struct AstListValueConst {
	/// `Value`+
	pub values: Vec<AstValueConst>,

	/// Node's location
	pub location: AstLocation,
}
