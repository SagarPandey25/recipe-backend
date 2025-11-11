use sea_orm::{prelude::Expr, sea_query::Func, *};

use crate::entities::{prelude::Recipe, recipe};

pub struct DBQuery;

impl DBQuery {
    pub async fn find_recipies(
        db: &DbConn,
        query: Option<String>,
        category: Option<String>,
    ) -> Result<Vec<recipe::Model>, DbErr> {
        let mut stmt = Recipe::find();

        if let Some(q) = query {
            let q = q.to_lowercase();
            stmt = stmt.filter(
                Expr::expr(Func::lower(Expr::col(recipe::Column::Title))).like(format!("%{}%", q)),
            )
        }

        if let Some(c) = category {
            stmt = stmt.filter(recipe::Column::Category.like(format!("%{}%", c)));
        }

        Ok(stmt.all(db).await?)
    }
}
