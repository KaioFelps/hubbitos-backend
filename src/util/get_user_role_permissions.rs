use crate::domain::domain_entities::role::Role;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RolePermissions {
    UpdateUser,
    ChangeUserPassword,

    CreateArticle,
    UpdateArticle,
    ApproveArticle,
    DisapproveArticle,
    DeleteArticle,

    InactivateComment,
    DeleteComment,

    SolveReport,
    DeleteReport,

    CreateNewTeamRole,
    UpdateTeamRole,
    DeleteTeamRole,

    CreateTeamUser,
    UpdateTeamUser,
    DeleteTeamUser,
}

impl RolePermissions {
    pub fn get_from_role(role: &Role) -> Vec<RolePermissions> {
        use RolePermissions::*;
    
        let perms_user = vec![];
        let perms_writter = vec![CreateArticle];
        let perms_editor = [&perms_writter[..], &[UpdateArticle, ApproveArticle]].concat();
        let perms_coord = [&perms_editor[..], &[DisapproveArticle, InactivateComment, SolveReport]].concat();
        let perms_admin = [&perms_coord[..], &[UpdateUser, DeleteComment, CreateTeamUser, UpdateTeamUser, DeleteTeamUser]].concat();
        let perms_principal = [&perms_admin[..], &[ChangeUserPassword, DeleteArticle, DeleteReport, CreateNewTeamRole, UpdateTeamRole]].concat();
        let perms_ceo = [&perms_principal[..], &[DeleteTeamRole]].concat();
    
        match role {
            Role::User => perms_user,
            Role::Writter => perms_writter,
            Role::Editor => perms_editor,
            Role::Coord => perms_coord,
            Role::Admin => perms_admin,
            Role::Principal => perms_principal,
            Role::Ceo => perms_ceo,
        }
    }
}

