//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub tid: i32,
    #[sea_orm(unique)]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::posttags::Entity")]
    Posttags,
}

impl Related<super::posttags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posttags.def()
    }
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        super::posttags::Relation::Posts.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::posttags::Relation::Tags.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::posttags::Entity")]
    Posttags,
    #[sea_orm(entity = "super::posts::Entity")]
    Posts,
}