//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "sys_login_log"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub info_id: String,
    pub login_name: String,
    pub net: String,
    pub ipaddr: String,
    pub login_location: String,
    pub browser: String,
    pub os: String,
    pub device: String,
    pub status: String,
    pub msg: String,
    pub login_time: DateTime,
    pub module: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    InfoId,
    LoginName,
    Net,
    Ipaddr,
    LoginLocation,
    Browser,
    Os,
    Device,
    Status,
    Msg,
    LoginTime,
    Module,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    InfoId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::InfoId => ColumnType::String(Some(32u32)).def(),
            Self::LoginName => ColumnType::String(Some(50u32)).def(),
            Self::Net => ColumnType::String(Some(10u32)).def(),
            Self::Ipaddr => ColumnType::String(Some(50u32)).def(),
            Self::LoginLocation => ColumnType::String(Some(255u32)).def(),
            Self::Browser => ColumnType::String(Some(50u32)).def(),
            Self::Os => ColumnType::String(Some(50u32)).def(),
            Self::Device => ColumnType::String(Some(50u32)).def(),
            Self::Status => ColumnType::Char(Some(1u32)).def(),
            Self::Msg => ColumnType::String(Some(255u32)).def(),
            Self::LoginTime => ColumnType::DateTime.def(),
            Self::Module => ColumnType::String(Some(30u32)).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
