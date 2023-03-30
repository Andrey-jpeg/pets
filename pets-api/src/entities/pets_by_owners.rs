//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "PetsByOwners")]
pub struct Model {
    #[sea_orm(column_name = "PetID", primary_key)]
    pub pet_id: i32,
    #[sea_orm(column_name = "PersonID", primary_key)]
    pub person_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::owners::Entity",
        from = "Column::PersonId",
        to = "super::owners::Column::PersonId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Owners,
    #[sea_orm(
        belongs_to = "super::pets::Entity",
        from = "Column::PetId",
        to = "super::pets::Column::PetId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Pets,
}

impl Related<super::owners::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owners.def()
    }
}

impl Related<super::pets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}