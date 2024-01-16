use crate::services::create_user_service::CreateUserService;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> CreateUserService {
    let sea_service = SeaService::new().await;

    let user_repository: SeaUserRepository = SeaUserRepository::new(sea_service).await;
    
    let create_user_service = CreateUserService::new(user_repository);

    create_user_service
}