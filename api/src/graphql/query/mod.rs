use entity::async_graphql;
pub use game::GameQuery;
pub use note::NoteQuery;
pub use questions::QuestionsQuery;
pub use user::UserQuery;

pub mod game;
pub mod note;
pub mod questions;
pub mod user;

// Add your other ones here to create a unified Query object
// e.x. Query(NoteQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UserQuery, NoteQuery, GameQuery, QuestionsQuery);

#[derive(async_graphql::MergedObject, Default)]
pub struct QueryAuth(NoteQuery);
