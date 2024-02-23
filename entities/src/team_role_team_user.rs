//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "team_role_team_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub team_role_id: Uuid,
    pub team_user_id: Uuid,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::team_role::Entity",
        from = "Column::TeamRoleId",
        to = "super::team_role::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    TeamRole,
    #[sea_orm(
        belongs_to = "super::team_user::Entity",
        from = "Column::TeamUserId",
        to = "super::team_user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    TeamUser,
}

impl Related<super::team_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamRole.def()
    }
}

impl Related<super::team_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}