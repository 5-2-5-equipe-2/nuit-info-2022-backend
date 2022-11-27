use entity::async_graphql;

pub mod note;
pub mod user;
pub use user::UserMutation;

pub use note::NoteMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(NoteMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(NoteMutation);
