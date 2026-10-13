mod common;

use common::migrated_database;
use rust_graphql_service_migrations::{Migrator, MigratorTrait};
use sea_orm_migration::{prelude::*, sea_orm::{DbBackend, Statement}};

#[tokio::test]
async fn down_drops_triggers_and_tables() {
    let database = migrated_database().await;
    Migrator::down(&database, None).await.unwrap();

    let row = database
        .query_one(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT COUNT(*) AS count FROM sqlite_master \
             WHERE name IN ('people_set_updated_at_on_update', \
                            'languages_set_updated_at_on_update', \
                            'people', 'languages')",
        ))
        .await
        .unwrap()
        .unwrap();
    let count: i64 = row.try_get("", "count").unwrap();
    assert_eq!(count, 0);
}