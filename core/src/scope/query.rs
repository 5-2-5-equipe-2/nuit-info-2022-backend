use sea_orm::*;

use ::entity::prelude::Scope;
use ::entity::scope;

pub struct Query;

impl Query {
    pub async fn find_scope_by_id(db: &DbConn, id: i32) -> Result<Option<scope::Model>, DbErr> {
        Scope::find_by_id(id).one(db).await
    }

    pub async fn get_all_scopes(db: &DbConn) -> Result<Vec<scope::Model>, DbErr> {
        Scope::find().all(db).await
    }

    /// If ok, returns (scope models, num pages).
    pub async fn find_scopes_in_page(
        db: &DbConn,
        page: u64,
        scopes_per_page: u64,
    ) -> Result<(Vec<scope::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Scope::find()
            .order_by_asc(scope::Column::Id)
            .paginate(db, scopes_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated scopes
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
