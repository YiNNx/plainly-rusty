use async_graphql::dynamic::{Schema, SchemaError};
use sea_orm::DatabaseConnection;
use seaography::{Builder, BuilderContext, GuardsConfig};

use super::guard::register::{entity_guards, field_guards};
use super::user::mutation_grant_token;
use crate::entities::*;

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext =custom_builder(BuilderContext::default());
}

fn custom_builder(context: BuilderContext) -> BuilderContext {
    BuilderContext {
        guards: GuardsConfig {
            entity_guards: entity_guards(),
            field_guards: field_guards(),
        },
        custom_query_fields: vec![],
        custom_mutation_fields: vec![mutation_grant_token()],
        ..context
    }
}

pub fn schema(database: DatabaseConnection) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    seaography::register_entities!(builder, [comments, post_tags, posts, tags]);
    builder.register_enumeration::<sea_orm_active_enums::CommentStatus>();
    builder.register_enumeration::<sea_orm_active_enums::PostStatus>();
    let schema = builder.schema_builder();
    schema.data(database).finish()
}
