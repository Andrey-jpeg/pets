//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "Owners")]
pub struct Model {
    #[sea_orm(column_name = "PersonID", primary_key, auto_increment = false)]
    pub person_id: i32,
    #[sea_orm(column_name = "FirstName")]
    pub first_name: String,
    #[sea_orm(column_name = "LastName")]
    pub last_name: String,
    #[sea_orm(column_name = "PhoneNumber")]
    pub phone_number: Option<String>,
    pub email: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::pets_by_owners::Entity")]
    PetsByOwners,
}

impl Related<super::pets_by_owners::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PetsByOwners.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
