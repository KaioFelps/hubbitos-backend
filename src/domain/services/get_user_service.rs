use uuid::Uuid;

use crate::domain::domain_entities::user::User;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::util::generate_service_internal_error;

pub struct GetUserServiceParams {
    pub user_id: Uuid
}

pub struct GetUserService<UserRepository: UserRepositoryTrait> {
    user_repository: Box<UserRepository>
}

impl<UserRepository: UserRepositoryTrait> GetUserService<UserRepository> {
    pub fn new(user_repository: Box<UserRepository>) -> Self {
        GetUserService {
            user_repository
        }
    }

    pub async fn exec(&self, params: GetUserServiceParams) -> Result<Option<User>, Box<dyn DomainErrorTrait>> {
        let user = self.user_repository.find_by_id(&params.user_id).await;

        if user.is_err() {
            return Err(generate_service_internal_error(
                "Error occurred on Get User Service, while selecting user by Id from the database",
                user.as_ref().unwrap_err()
            ));
        }

        let user = user.unwrap();
        Ok(user)
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::role::Role;
    use crate::domain::repositories::user_repository::MockUserRepositoryTrait;

    use super::*;
    use std::sync::Mutex;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_get_user_service() {
        let user = User::new("Kaio".into(), "12345".into(), Some(Role::Coord));

        // mocked db
        let user_db: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));
        user_db.lock().unwrap().push(user.clone());
        
        // mocking user repository
        let mut mocked_user_repository = MockUserRepositoryTrait::new();

        let db_clone = Arc::clone(&user_db);
        mocked_user_repository
        .expect_find_by_id()
        .returning(move |id| {
            let mut found_user: Option<User> = None;
            
            for user in db_clone.lock().unwrap().clone().into_iter() {
                if user.id().eq(id) {
                    found_user = Some(user);
                }
            }

            Ok(found_user)
        });

        // TESTING
        let sut = GetUserService::new(Box::new(mocked_user_repository));

        let success_result = sut.exec(GetUserServiceParams { user_id: user.id() }).await.unwrap();

        assert!(success_result.is_some());

        let failling_result = sut.exec(GetUserServiceParams { user_id: Uuid::new_v4() }).await.unwrap();

        assert!(failling_result.is_none());
    }
}
