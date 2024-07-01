use crate::AstDirectivesConst;
use crate::AstFieldsDefinition;
use crate::AstImplementsInterfaces;
use crate::AstLocation;
use crate::AstName;

/// ObjectTypeExtension :
///   extend type `Name` `ImplementsInterfaces`? `Directives[Const]`? `FieldsDefinition`
///   extend type `Name` `ImplementsInterfaces`? `Directives[Const]` [lookahead != {\]
///   extend type `Name` `ImplementsInterfaces` [lookahead != {\]
///
/// Spec: https://spec.graphql.org/draft/#ObjectTypeExtension
#[derive(Clone, Debug)]
pub struct AstObjectTypeExtension {
	/// `Name`
	pub name: AstName,

	/// `ImplementsInterfaces`?
	pub implements: Option<AstImplementsInterfaces>,

	/// `Directives[Const]`
	pub directives: Option<AstDirectivesConst>,

	/// `FieldsDefinition`
	/// [lookahead != {\]
	pub fields: Option<AstFieldsDefinition>,

	/// Node's location
	pub location: AstLocation,
}
