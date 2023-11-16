use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "posts")]
#[graphql(complex)]
#[graphql(name = "Posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub pid: i32,
    pub title: String,
    pub time: Option<DateTime>,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub views: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub summary: Option<String>,
    pub status: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::comments::Entity")]
    Comments,
    #[sea_orm(has_many = "super::posttags::Entity")]
    Posttags,
}

impl Related<super::comments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<super::posttags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posttags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
