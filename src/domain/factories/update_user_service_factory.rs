use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;
use crate::domain::services::update_user_service::UpdateUserService;

pub async fn exec() -> UpdateUserService<SeaUserRepository> {
    let sea_service = SeaService::new().await;

    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service).await);

    let hasher = Box::new(PasswordAuthHasherAndVerifier {});
    
    let update_user_service = UpdateUserService::new(user_repository, hasher);

    update_user_service
}