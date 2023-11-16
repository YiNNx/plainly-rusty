use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "tags")]
#[graphql(complex)]
#[graphql(name = "Tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub tid: i32,
    #[sea_orm(unique)]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::posttags::Entity")]
    Posttags,
}

impl Related<super::posttags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posttags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
