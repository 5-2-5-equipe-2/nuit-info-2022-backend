use entity::async_graphql;
pub use game::GameMutation;
pub use note::NoteMutation;
pub use questions::QuestionsMutation;
pub use scope::ScopeMutation;
pub use user::UserMutation;

pub mod game;
pub mod note;
pub mod questions;
pub mod scope;
pub mod user;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(NoteMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(
    UserMutation,
    ScopeMutation,
    GameMutation,
    NoteMutation,
    QuestionsMutation,
);

#[derive(async_graphql::MergedObject, Default)]
pub struct MutationAuth(NoteMutation);
