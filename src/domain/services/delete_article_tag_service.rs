use crate::domain::domain_entities::role::Role;
use crate::domain::repositories::article_tag_repository::ArticleTagRepositoryTrait;
use crate::errors::error::DomainErrorTrait;
use crate::errors::unauthorized_error::UnauthorizedError;
use crate::util::{generate_service_internal_error, RolePermissions, verify_role_has_permission};

pub struct DeleteArticleTagParams<'run> {
    pub user_role: &'run Role,
    pub tag_id: i32,
}

pub struct DeleteArticleTagService<ArticleTagRepository: ArticleTagRepositoryTrait> {
    article_tag_repository: ArticleTagRepository
}

impl<ArticleTagRepository: ArticleTagRepositoryTrait> DeleteArticleTagService<ArticleTagRepository> {
    pub fn new(article_tag_repository: ArticleTagRepository) -> Self {
        DeleteArticleTagService {
            article_tag_repository,
        }
    }

    pub async fn exec<'run>(&self, params: DeleteArticleTagParams<'run>) -> Result<(), Box<dyn DomainErrorTrait>> {
        let user_can_delete_article_tag = verify_role_has_permission(params.user_role, RolePermissions::DeleteArticleTag);

        if !user_can_delete_article_tag {
            return Err(Box::new(UnauthorizedError::new()));
        }

        let tag = self.article_tag_repository.find_by_id(params.tag_id).await;

        if tag.is_err() {
            return Err(generate_service_internal_error(
                &format!("Error occurred in Delete Article Tag Service on finding tag with id {}.", params.tag_id).to_string(),
                &tag.unwrap_err(),
            ));
        }

        let tag = tag.unwrap();

        if tag.is_none() {
            return Ok(())
        }

        let result = self.article_tag_repository.delete(tag.unwrap()).await;

        if result.is_err() {
            return Err(generate_service_internal_error(
                "Error occurred in Delete Article Tag Service on deleting the tag from the database".into(),
                &result.unwrap_err()
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::domain::domain_entities::article_tag::ArticleTag;
    use crate::domain::domain_entities::role::Role;
    use crate::domain::services::delete_article_tag_service::DeleteArticleTagParams;
    use crate::tests::repositories::article_tag_repository::get_article_tag_repository;

    #[tokio::test]
    async fn test_if_staff_can_delete_article_tag() {
        let (tag_db, tag_repository) = get_article_tag_repository();
        let sut = super::DeleteArticleTagService::new(tag_repository);

        tag_db.lock().unwrap().push(ArticleTag::new_from_existing(1, "Foo".into()));

        let response = sut.exec(DeleteArticleTagParams {
            tag_id: 1,
            user_role: &Role::Ceo
        }).await;

        assert!(response.is_ok());
        assert_eq!(0, tag_db.lock().unwrap().len(), "Expected database to be empty after the successful delete of the article tag.");
    }

    #[tokio::test]
    async fn test_if_non_authorized_user_cannot_delete_article_tag() {
        let (tag_db, tag_repository) = get_article_tag_repository();
        let sut = super::DeleteArticleTagService::new(tag_repository);

        tag_db.lock().unwrap().push(ArticleTag::new_from_existing(1, "Foo".into()));

        let response = sut.exec(DeleteArticleTagParams {
            tag_id: 1,
            user_role: &Role::Principal
        }).await;

        assert!(response.is_err());
        assert_eq!(1, tag_db.lock().unwrap().len(), "Expected database not to be empty after delete request being rejected.");
    }
}
