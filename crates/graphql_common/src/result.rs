use crate::error::GraphQLError;

///
pub type GraphQLResult<T> = Result<T, GraphQLError>;
