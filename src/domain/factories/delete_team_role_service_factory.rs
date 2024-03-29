use crate::domain::services::delete_team_role_service::DeleteTeamRoleService;
use crate::infra::sea::repositories::sea_team_role_repository::SeaTeamRoleRepository;
use crate::infra::sea::sea_service::SeaService;

pub async fn exec() -> DeleteTeamRoleService<SeaTeamRoleRepository> {
    let sea_service = SeaService::new().await;

    let team_role_repository: Box<SeaTeamRoleRepository> = Box::new(SeaTeamRoleRepository::new(sea_service).await);
    
    let delete_team_role_service = DeleteTeamRoleService::new(
        team_role_repository
    );

    delete_team_role_service
}