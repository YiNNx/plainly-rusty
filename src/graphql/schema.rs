use async_graphql::dynamic::{Schema, SchemaError};
use sea_orm::DatabaseConnection;
use seaography::{register_entities, Builder, BuilderContext, GuardsConfig};

use super::custom_resolver::{comment, grant_token};
use super::guard::guard_public;
use super::guard::{entity_guards, field_guards};
use crate::entities::{comments, post_tags, posts, sea_orm_active_enums::*, tags};

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext =custom_builder(BuilderContext::default());
}

fn custom_builder(context: BuilderContext) -> BuilderContext {
    BuilderContext {
        guards: GuardsConfig {
            entity_guards: entity_guards(vec![
                ("Posts", Box::new(guard_public)),
                ("Tags", Box::new(guard_public)),
                ("PostTags", Box::new(guard_public)),
                ("Comments", Box::new(guard_public)),
            ]),
            field_guards: field_guards(vec![]),
        },
        custom_query_fields: vec![],
        custom_mutation_fields: vec![grant_token(), comment()],
        ..context
    }
}

pub fn schema(database: DatabaseConnection) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    register_entities!(builder, [comments, post_tags, posts, tags]);
    builder.register_enumeration::<CommentStatus>();
    builder.register_enumeration::<PostStatus>();
    let schema = builder.schema_builder();
    schema.data(database).finish()
}
