use std::error::Error;
use uuid::Uuid;

use crate::domain::repositories::article_repository::ArticleRepositoryTrait;
use crate::domain::repositories::user_repository::UserRepositoryTrait;
use crate::errors::resource_not_found::ResourceNotFoundError;
use crate::errors::{internal_error::InternalError, unauthorized_error::UnauthorizedError};
use crate::util::{RolePermissions, verify_role_has_permission};

pub struct DeleteArticleParams {
    pub user_id: Uuid,
    pub article_id: Uuid,
}
pub struct DeleteArticleService<
ArticleRepository: ArticleRepositoryTrait,
UserRepository: UserRepositoryTrait
> {
    user_repository: Box<UserRepository>,
    article_repository: Box<ArticleRepository>,
}

impl
<ArticleRepository: ArticleRepositoryTrait,
UserRepository: UserRepositoryTrait>
DeleteArticleService<ArticleRepository, UserRepository>
{
    pub fn new(article_repository: Box<ArticleRepository>, user_repository: Box<UserRepository>) -> Self {
        DeleteArticleService {
            article_repository,
            user_repository,
        }
    }

    pub async fn exec(&self, params: DeleteArticleParams) -> Result<(), Box<dyn Error>> {
        let user_on_db = &self.user_repository.find_by_id(&params.user_id).await;

        if user_on_db.is_err() { return Err(Box::new(InternalError::new())); }

        let user_on_db = user_on_db.as_ref().unwrap().to_owned();

        if user_on_db.is_none() { return Err(Box::new(UnauthorizedError::new())) }

        // article verifications

        let article_on_db = &self.article_repository.find_by_id(params.article_id).await;

        if article_on_db.is_err() { return Err(Box::new(InternalError::new())); }
        
        let article_on_db = article_on_db.as_ref().unwrap();

        if article_on_db.is_none() { return Err(Box::new(ResourceNotFoundError::new())) }

        let article = article_on_db.clone().unwrap();

        // checks user is allowed to perform the update
        let user_can_delete = verify_role_has_permission(
            &user_on_db.as_ref().unwrap().role().unwrap().clone().to_owned(),
            RolePermissions::DeleteArticle
        );

        if !user_can_delete { return Err(Box::new(UnauthorizedError::new())); }

        let response = &self.article_repository.delete(article).await;

        if response.as_ref().is_ok() {
            return Ok(());
        }

        else {
           return Err(Box::new(InternalError::new()));
        }
    }
}


#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;
    use tokio;

    use super::{DeleteArticleParams, DeleteArticleService};

    use crate::domain::repositories::user_repository::MockUserRepositoryTrait;
    use crate::domain::repositories::article_repository::MockArticleRepositoryTrait;
    use crate::domain::domain_entities::user::User;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::domain_entities::article::Article;

    #[tokio::test]
    async fn test() {
        let mut mocked_user_repo: MockUserRepositoryTrait = MockUserRepositoryTrait::new();
        let mut mocked_article_repo: MockArticleRepositoryTrait = MockArticleRepositoryTrait::new();

        let article = Article::new(
            Uuid::new_v4(),
            "Título inicial".to_string(),
            "Conteúdo inicial".to_string(),
            "coverurl.inicial".to_string()
        );

        let article_db: Arc<Mutex<Vec<Article>>> = Arc::new(Mutex::new(vec![
            article.clone()
        ]));
        
        // mocking user repo
        mocked_user_repo
        .expect_find_by_id()
        .returning(|id| {
            let fake_user = User::new_from_existing(
                id.clone().to_owned(),
                "Fake name".to_string(),
                "password".to_string(),
                chrono::Utc::now().naive_utc(),
                None,
                Some(Role::Principal)
            );

            Ok(Some(fake_user))
        });

        // mocking article repo
    
        let mocked_article_repo_db_clone = Arc::clone(&article_db);
        mocked_article_repo
        .expect_find_by_id()
        .returning(move |id| {
            let article_db = mocked_article_repo_db_clone.lock().unwrap();

            for item in article_db.iter() {
                if item.id() == id {
                    return Ok(Some(item.clone()));
                }
            }

            Ok(None)
        });

        let mocked_article_repo_db_clone = Arc::clone(&article_db);
        mocked_article_repo
        .expect_delete()
        .returning(move |_article| {
            let mut article_db = mocked_article_repo_db_clone.lock().unwrap();
            article_db.truncate(0);

            Ok(())
        });

        let service = DeleteArticleService {
            user_repository: Box::new(mocked_user_repo),
            article_repository: Box::new(mocked_article_repo)
        };

        let result = service.exec(DeleteArticleParams {
            user_id: article.author_id(),
            article_id: article.id(),
        });

        tokio::try_join!(result).unwrap();

        let db = article_db.lock().unwrap();
        assert_eq!(0, db.len());
    }
}