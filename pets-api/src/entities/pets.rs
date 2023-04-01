//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "Pets")]
pub struct Model {
    #[sea_orm(column_name = "PetID", primary_key, auto_increment = false)]
    pub pet_id: i32,
    #[sea_orm(column_name = "PetName")]
    pub pet_name: String,
    #[sea_orm(column_name = "Color")]
    pub color: Option<String>,
    #[sea_orm(column_name = "Status")]
    pub status: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::pet_location::Entity")]
    PetLocation,
}

impl Related<super::pet_location::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PetLocation.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_pets::Relation::Users.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_pets::Relation::Pets.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
