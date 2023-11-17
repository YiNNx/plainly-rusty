//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cid: i32,
    pub parent_cid: Option<i32>,
    pub post_id: Option<i32>,
    pub username: Option<String>,
    pub time: Option<DateTime>,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub status: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentCid",
        to = "Column::Cid",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef,
    #[sea_orm(
        belongs_to = "super::posts::Entity",
        from = "Column::PostId",
        to = "super::posts::Column::Pid",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Posts,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::Username",
        to = "super::users::Column::Username",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "Entity", def = "Relation::SelfRef.def()")]
    SelfRef,
    #[sea_orm(entity = "super::posts::Entity")]
    Posts,
    #[sea_orm(entity = "super::users::Entity")]
    Users,
    #[sea_orm(entity = "Entity", def = "Relation::SelfRef.def().rev()")]
    SelfRefReverse,
}