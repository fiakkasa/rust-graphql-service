use rust_graphql_service::query_root::schema;
use rust_graphql_service_migrations::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use seaography::async_graphql::dynamic::Schema;

pub async fn setup() -> (DatabaseConnection, Schema) {
    let database = Database::connect("sqlite::memory:").await.unwrap();
    Migrator::up(&database, None).await.unwrap();
    database
        .execute_unprepared(
            "INSERT INTO languages (name, iso_639_1, iso_639_2, updated_at) \
             VALUES ('English', 'en', 'eng', '2000-01-01 00:00:00');",
        )
        .await
        .unwrap();
    database
        .execute_unprepared(
            "INSERT INTO people (first_name, last_name, language_id, favorite_color, updated_at) \
             VALUES ('Ada', 'Lovelace', 1, 'Red', '2000-01-01 00:00:00');",
        )
        .await
        .unwrap();

    let schema = schema(database.clone(), 16, 4096).unwrap();
    (database, schema)
}

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