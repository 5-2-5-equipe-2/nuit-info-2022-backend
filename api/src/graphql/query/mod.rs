use entity::async_graphql;
pub use note::NoteQuery;
pub use user::UserQuery;

pub mod note;
pub mod user;

// Add your other ones here to create a unified Query object
// e.x. Query(NoteQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UserQuery);

#[derive(async_graphql::MergedObject, Default)]
pub struct QueryAuth(NoteQuery);
