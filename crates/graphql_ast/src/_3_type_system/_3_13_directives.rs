use crate::AstArgumentsDefinition;
use crate::AstDescription;
use crate::AstLocation;
use crate::AstName;

/// DirectiveDefinition :
///   `Description`? directive @ `Name` `ArgumentsDefinition`? repeatable? on `DirectiveLocations`
///
/// Spec: https://spec.graphql.org/draft/#DirectiveDefinition
#[derive(Clone, Debug)]
pub struct AstDirectiveDefinition {
	/// `Description`?
	pub description: Option<AstDescription>,

	/// `Name`
	pub name: AstName,

	/// `ArgumentsDefinition`?
	pub arguments: Option<AstArgumentsDefinition>,

	/// repeatable?
	pub repeatable: bool,

	/// `DirectiveLocations`
	pub locations: AstDirectiveLocations,

	/// Node's location
	pub location: AstLocation,
}

/// DirectiveLocations :
///   `DirectiveLocations` | `DirectiveLocation`
///   |? `DirectiveLocation`
///
/// Spec: https://spec.graphql.org/draft/#DirectiveLocations
#[derive(Clone, Debug)]
pub struct AstDirectiveLocations {
	/// `DirectiveLocations` | `DirectiveLocation`
	/// |? `DirectiveLocation`
	pub locations: Vec<AstDirectiveLocation>,

	/// Node's location
	pub location: AstLocation,
}

/// DirectiveLocation :
///   `ExecutableDirectiveLocation`
///   `TypeSystemDirectiveLocation`
///
/// Spec: https://spec.graphql.org/draft/#DirectiveLocation
#[derive(Clone, Debug)]
pub enum AstDirectiveLocation {
	/// `ExecutableDirectiveLocation`
	ExecutableDirectiveLocation(AstExecutableDirectiveLocation),

	/// `TypeSystemDirectiveLocation`
	TypeSystemDirectiveLocation(AstTypeSystemDirectiveLocation),
}

/// ExecutableDirectiveLocation : one of
///   QUERY
///   MUTATION
///   SUBSCRIPTION
///   FIELD
///   FRAGMENT_DEFINITION
///   FRAGMENT_SPREAD
///   INLINE_FRAGMENT
///   VARIABLE_DEFINITION
///
/// Spec: https://spec.graphql.org/draft/#ExecutableDirectiveLocation
#[derive(Clone, Debug)]
pub enum AstExecutableDirectiveLocation {
	/// QUERY
	Query(AstLocation),

	/// MUTATION
	Mutation(AstLocation),

	/// SUBSCRIPTION
	Subscription(AstLocation),

	/// FIELD
	Field(AstLocation),

	/// FRAGMENT_DEFINITION
	FragmentDefinition(AstLocation),

	/// FRAGMENT_SPREAD
	FragmentSpread(AstLocation),

	/// INLINE_FRAGMENT
	InlineFragment(AstLocation),

	/// VARIABLE_DEFINITION
	VariableDefinition(AstLocation),
}

/// TypeSystemDirectiveLocation :
///   SCHEMA
///   SCALAR
///   OBJECT
///   FIELD_DEFINITION
///   ARGUMENT_DEFINITION
///   INTERFACE
///   UNION
///   ENUM
///   ENUM_VALUE
///   INPUT_OBJECT
///   INPUT_FIELD_DEFINITION
///
/// Spec: https://spec.graphql.org/draft/#TypeSystemDirectiveLocation
#[derive(Clone, Debug)]
pub enum AstTypeSystemDirectiveLocation {
	/// SCHEMA
	Schema(AstLocation),

	/// SCALAR
	Scalar(AstLocation),

	/// OBJECT
	Object(AstLocation),

	/// FIELD_DEFINITION
	FieldDefinition(AstLocation),

	/// ARGUMENT_DEFINITION
	ArgumentDefinition(AstLocation),

	/// INTERFACE
	Interface(AstLocation),

	/// UNION
	Union(AstLocation),

	/// ENUM
	Enum(AstLocation),

	/// ENUM_VALUE
	EnumValue(AstLocation),

	/// INPUT_OBJECT
	InputObject(AstLocation),

	/// INPUT_FIELD_DEFINITION
	InputFieldDefinition(AstLocation),
}
