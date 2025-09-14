use sea_orm::{prelude::Expr, sea_query::Func, *};

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
            stmt = stmt.filter(
                Expr::expr(Func::lower(Expr::col(recipe::Column::Title))).like(format!("%{}%", q)),
            )
        }

        Ok(stmt.all(db).await?)
    }
}
