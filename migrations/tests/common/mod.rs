use rust_graphql_service_migrations::{Migrator, MigratorTrait};
use sea_orm_migration::{
    prelude::*,
    sea_orm::{Database, DatabaseConnection, DbBackend, Statement},
};

pub async fn migrated_database() -> DatabaseConnection {
    let database = Database::connect("sqlite::memory:").await.unwrap();
    Migrator::up(&database, None).await.unwrap();
    database
}

#[allow(dead_code)]
pub async fn insert_english(database: &DatabaseConnection) {
    database
        .execute_unprepared(
            "INSERT INTO languages (name, iso_639_1, iso_639_2) \
             VALUES ('English', 'en', 'eng');",
        )
        .await
        .unwrap();
}

#[allow(dead_code)]
pub async fn updated_at(database: &DatabaseConnection, table: &str) -> Option<String> {
    let row = database
        .query_one(Statement::from_string(
            DbBackend::Sqlite,
            format!("SELECT updated_at FROM {table} WHERE id = 1"),
        ))
        .await
        .unwrap()
        .unwrap();
    row.try_get("", "updated_at").unwrap()
}