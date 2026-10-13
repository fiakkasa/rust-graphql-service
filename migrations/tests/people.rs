mod common;

use common::{insert_english, migrated_database, updated_at};
use sea_orm_migration::{prelude::*, sea_orm::{DbBackend, Statement}};

#[tokio::test]
async fn update_trigger_refreshes_person_timestamp() {
    let database = migrated_database().await;
    insert_english(&database).await;
    database
        .execute_unprepared(
            "INSERT INTO people (first_name, last_name, language_id) \
             VALUES ('Ada', 'Lovelace', 1);",
        )
        .await
        .unwrap();

    database
        .execute_unprepared("UPDATE people SET first_name = 'Augusta' WHERE id = 1;")
        .await
        .unwrap();

    let timestamp = updated_at(&database, "people").await;
    assert!(timestamp.is_some());
    assert_ne!(timestamp.as_deref(), Some("2000-01-01 00:00:00"));
}

#[tokio::test]
async fn names_require_three_to_128_characters() {
    let database = migrated_database().await;
    insert_english(&database).await;

    for (first_name, last_name) in [
        ("Al".to_string(), "Lovelace".to_string()),
        ("Ada".to_string(), "Li".to_string()),
        ("A".repeat(129), "Lovelace".to_string()),
        ("Ada".to_string(), "L".repeat(129)),
    ] {
        let statement = Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "INSERT INTO people (first_name, last_name, language_id) VALUES (?, ?, 1)",
            [first_name.into(), last_name.into()],
        );
        assert!(database.execute(statement).await.is_err());
    }

    for (first_name, last_name) in [
        ("Ada".to_string(), "Lovelace".to_string()),
        ("A".repeat(128), "L".repeat(128)),
    ] {
        let statement = Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "INSERT INTO people (first_name, last_name, language_id) VALUES (?, ?, 1)",
            [first_name.into(), last_name.into()],
        );
        database.execute(statement).await.unwrap();
    }
}