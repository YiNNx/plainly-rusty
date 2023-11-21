use async_graphql::dynamic::{Schema, SchemaError};
use sea_orm::DatabaseConnection;
use seaography::{Builder, BuilderContext};

use super::guard::register::register_guard;
use crate::entities::*;

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext =register_guard(BuilderContext::default());
}

pub fn schema(database: DatabaseConnection) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    seaography::register_entities!(builder, [comments, post_tags, posts, tags, users,]);
    builder.register_enumeration::<sea_orm_active_enums::CommentStatus>();
    builder.register_enumeration::<sea_orm_active_enums::PostStatus>();
    builder.register_enumeration::<sea_orm_active_enums::UserRole>();
    let schema = builder.schema_builder();
    schema.data(database).finish()
}
