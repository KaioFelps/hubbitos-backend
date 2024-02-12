pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240114_032712_add_roles_to_user;
mod m20240125_212235_add_new_fields_to_articles;
mod m20240128_044449_make_article_content_not_nullable;
mod m20240128_070407_add_approved_field_to_article;
mod m20240204_210351_add_slug_to_article;
mod m20240209_010037_setup_comments_table;
mod m20240212_051303_drop_comment_article_table;
mod m20240212_051315_add_article_id_field_to_comment_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240114_032712_add_roles_to_user::Migration),
            Box::new(m20240125_212235_add_new_fields_to_articles::Migration),
            Box::new(m20240128_044449_make_article_content_not_nullable::Migration),
            Box::new(m20240128_070407_add_approved_field_to_article::Migration),
            Box::new(m20240204_210351_add_slug_to_article::Migration),
            Box::new(m20240209_010037_setup_comments_table::Migration),
            Box::new(m20240212_051303_drop_comment_article_table::Migration),
            Box::new(m20240212_051315_add_article_id_field_to_comment_table::Migration),
        ]
    }
}
