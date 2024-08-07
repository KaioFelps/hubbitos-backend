use crate::domain::services::create_user_service::CreateUserService;
use actix_web::HttpResponse;
use either::Either::{self, *};
use crate::errors::internal_error::InternalError;
use crate::infra::cryptography::PasswordAuthHasherAndVerifier;
use crate::infra::sea::sea_service::SeaService;
use crate::infra::sea::repositories::sea_user_repository::SeaUserRepository;

pub async fn exec() -> Either<CreateUserService<SeaUserRepository>, HttpResponse> {
    let sea_service = SeaService::new().await;

    if sea_service.is_err() {
        return Right(crate::util::generate_error_response(Box::new(InternalError::new())))
    }

    let sea_service = sea_service.unwrap();

    let user_repository: Box<SeaUserRepository> = Box::new(SeaUserRepository::new(sea_service).await);

    let hasher = Box::new(PasswordAuthHasherAndVerifier {});
    
    let create_user_service = CreateUserService::new(user_repository, hasher);

    Left(create_user_service)
}