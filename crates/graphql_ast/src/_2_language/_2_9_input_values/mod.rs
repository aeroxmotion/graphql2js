use super::_2_10_variables::AstVariable;

pub mod _2_9_1_int_value;
pub mod _2_9_2_float_value;
pub mod _2_9_3_boolean_value;
pub mod _2_9_4_string_value;
pub mod _2_9_5_null_value;
pub mod _2_9_6_enum_value;
pub mod _2_9_7_list_value;
pub mod _2_9_8_input_object_values;

pub use _2_9_1_int_value::*;
pub use _2_9_2_float_value::*;
pub use _2_9_3_boolean_value::*;
pub use _2_9_4_string_value::*;
pub use _2_9_5_null_value::*;
pub use _2_9_6_enum_value::*;
pub use _2_9_7_list_value::*;
pub use _2_9_8_input_object_values::*;

/// Value :
///   `Variable`
///   `IntValue`
///   `FloatValue`
///   `StringValue`
///   `BooleanValue`
///   `NullValue`
///   `EnumValue`
///   `ListValue`
///   `ObjectValue`
///
/// Spec: https://spec.graphql.org/draft/#Value
#[derive(Clone, Debug)]
pub enum AstValue {
	/// `Variable`
	Variable(AstVariable),

	/// `IntValue`
	IntValue(AstIntValue),

	/// `FloatValue`
	FloatValue(AstFloatValue),

	/// `StringValue`
	StringValue(AstStringValue),

	/// `BooleanValue`
	BooleanValue(AstBooleanValue),

	/// `NullValue`
	NullValue(AstNullValue),

	/// `EnumValue`
	EnumValue(AstEnumValue),

	/// `ListValue`
	ListValue(Box<AstListValue>),

	/// `ObjectValue`
	ObjectValue(Box<AstObjectValue>),
}

/// Value[Const] :
///   `IntValue`
///   `FloatValue`
///   `StringValue`
///   `BooleanValue`
///   `NullValue`
///   `EnumValue`
///   `ListValue[Const]`
///   `ObjectValue[Const]`
///
/// Spec: https://spec.graphql.org/draft/#Value
#[derive(Clone, Debug)]
pub enum AstValueConst {
	/// `IntValue`
	IntValue(AstIntValue),

	/// `FloatValue`
	FloatValue(AstFloatValue),

	/// `StringValue`
	StringValue(AstStringValue),

	/// `BooleanValue`
	BooleanValue(AstBooleanValue),

	/// `NullValue`
	NullValue(AstNullValue),

	/// `EnumValue`
	EnumValue(AstEnumValue),

	/// `ListValue[Const]`
	ListValueConst(Box<AstListValueConst>),

	/// `ObjectValue[Const]`
	ObjectValueConst(Box<AstObjectValueConst>),
}
