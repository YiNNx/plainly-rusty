use async_graphql::dynamic::{Schema, SchemaError};
use async_graphql::{dynamic::*, Value};
use sea_orm::DatabaseConnection;
use seaography::{Builder, BuilderContext, CustomMutation, CustomMutationArgument, GuardsConfig};

use super::guard::register::{entity_guards, field_guards};
use crate::entities::*;

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext =custom_builder(BuilderContext::default());
}

struct Response {
    code: String,
}

fn custom_builder(context: BuilderContext) -> BuilderContext {
    BuilderContext {
        guards: GuardsConfig {
            entity_guards: entity_guards(),
            field_guards: field_guards(),
        },
        custom_query_fields: vec![],
        custom_mutation_fields: vec![CustomMutation {
            name: "getUserToken".into(),
            ty: TypeRef::named_nn(TypeRef::STRING),
            resolver_fn: Box::new(|_ctx| {
                FieldFuture::new(async move { Ok(Some(Value::from("test"))) })
            }),
            arguments: vec![CustomMutationArgument {
                name: "token".into(),
                ty: TypeRef::named_nn(TypeRef::STRING),
            }],
        }],
        ..context
    }
}

pub fn schema(database: DatabaseConnection) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    // let query = Object::new("Test").field(Field::new(
    //     "value",
    //     TypeRef::named_nn(TypeRef::STRING),
    //     |ctx| FieldFuture::new(async move { Ok(Some(Value::from("abc"))) }),
    // ));
    // builder.schema = builder.schema.register(query);
    seaography::register_entities!(builder, [comments, post_tags, posts, tags, users]);
    builder.register_enumeration::<sea_orm_active_enums::CommentStatus>();
    builder.register_enumeration::<sea_orm_active_enums::PostStatus>();
    builder.register_enumeration::<sea_orm_active_enums::UserRole>();
    let schema = builder.schema_builder();
    schema.data(database).finish()
}
