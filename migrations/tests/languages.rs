mod common;

use common::{insert_english, migrated_database, updated_at};
use sea_orm_migration::{prelude::*, sea_orm::{DbBackend, Statement}};

#[tokio::test]
async fn update_trigger_refreshes_language_timestamp() {
    let database = migrated_database().await;
    insert_english(&database).await;

    database
        .execute_unprepared("UPDATE languages SET name = 'English updated' WHERE id = 1;")
        .await
        .unwrap();

    let timestamp = updated_at(&database, "languages").await;
    assert!(timestamp.is_some());
    assert_ne!(timestamp.as_deref(), Some("2000-01-01 00:00:00"));
}

#[tokio::test]
async fn iso_codes_require_exact_lengths_and_no_spaces() {
    let database = migrated_database().await;
    for (iso6391, iso6392) in [("e", "eng"), ("e ", "eng"), ("en", "en"), ("en", "e g")] {
        let statement = Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "INSERT INTO languages (name, iso_639_1, iso_639_2) VALUES ('Invalid', ?, ?)",
            [iso6391.to_string().into(), iso6392.to_string().into()],
        );
        assert!(
            database.execute(statement).await.is_err(),
            "accepted invalid ISO codes {iso6391:?} and {iso6392:?}"
        );
    }
}

#[tokio::test]
async fn name_requires_three_to_128_characters() {
    let database = migrated_database().await;

    for (name, iso6391, iso6392) in [
        ("ab".to_string(), "en", "eng"),
        ("A".repeat(129), "fr", "fra"),
    ] {
        let character_count = name.chars().count();
        let statement = Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "INSERT INTO languages (name, iso_639_1, iso_639_2) VALUES (?, ?, ?)",
            [name.into(), iso6391.into(), iso6392.into()],
        );
        assert!(
            database.execute(statement).await.is_err(),
            "accepted a language name of {character_count} characters"
        );
    }

    for (name, iso6391, iso6392) in [
        ("eng".to_string(), "en", "eng"),
        ("A".repeat(128), "fr", "fra"),
    ] {
        let statement = Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "INSERT INTO languages (name, iso_639_1, iso_639_2) VALUES (?, ?, ?)",
            [name.into(), iso6391.into(), iso6392.into()],
        );
        database.execute(statement).await.unwrap();
    }
}