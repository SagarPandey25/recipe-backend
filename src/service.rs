use sea_orm::*;

use crate::entities::{prelude::Recipe, recipe};

pub struct DBQuery;

impl DBQuery {
    pub async fn find_recipies(
        db: &DbConn,
        query: Option<String>,
    ) -> Result<Vec<recipe::Model>, DbErr> {
        let mut stmt = Recipe::find();

        if let Some(q) = query {
            let q = q.to_lowercase();
            stmt = stmt.filter(recipe::Column::Title.contains(q));
        }

        Ok(stmt.all(db).await?)
    }
}
