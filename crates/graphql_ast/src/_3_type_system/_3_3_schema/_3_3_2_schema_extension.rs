use graphql_common::location::AstLocation;

use crate::AstDirectives;
use crate::AstRootOperationTypeDefinition;

/// SchemaExtension :
///   extend schema `Directives[Const]`? { `RootOperationTypeDefinition`+ }
///   extend schema `Directives[Const]` [lookahead != {\]
///
/// Spec: https://spec.graphql.org/draft/#SchemaExtension
#[derive(Clone, Debug)]
pub struct AstSchemaExtension {
	/// `Directives[Const]`?
	pub directives: Option<AstDirectives>,

	/// `RootOperationTypeDefinition`+
	/// [lookahead != {\]
	pub definitions: Option<AstRootOperationTypeDefinition>,

	/// Node's location
	pub location: AstLocation,
}
