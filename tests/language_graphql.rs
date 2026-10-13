mod common;

use common::{setup, updated_at};
use seaography::async_graphql::Request;

#[tokio::test]
async fn languages_update_refreshes_updated_at() {
    let (database, schema) = setup().await;
    let response = schema
        .execute(Request::new(
            "mutation { languagesUpdate(data: { name: \"English updated\" }, filter: { id: { eq: 1 } }) { id } }",
        ))
        .await;

    assert!(response.errors.is_empty(), "{:?}", response.errors);
    let timestamp = updated_at(&database, "languages").await;
    assert!(timestamp.is_some());
    assert_ne!(timestamp.as_deref(), Some("2000-01-01 00:00:00"));
}

#[tokio::test]
async fn languages_mutations_validate_name_and_iso_codes() {
    let (_, schema) = setup().await;

    for (name, iso6391, iso6392) in [
        ("Eng".to_string(), "fr", "fra"),
        ("A".repeat(128), "fr", "fra"),
    ] {
        let mutation = format!(
            "mutation {{ languagesUpdate(data: {{ name: \"{name}\", iso6391: \"{iso6391}\", iso6392: \"{iso6392}\" }}, filter: {{ id: {{ eq: 1 }} }}) {{ id }} }}"
        );
        let response = schema.execute(Request::new(mutation)).await;
        assert!(response.errors.is_empty(), "{:?}", response.errors);
    }

    for (name, iso6391, iso6392) in [
        ("ab", "e ", "f r"),
        ("A", "eng", "fr"),
        ("A".repeat(129).as_str(), "en", "eng"),
    ] {
        let mutation = format!(
            "mutation {{ languagesUpdate(data: {{ name: \"{name}\", iso6391: \"{iso6391}\", iso6392: \"{iso6392}\" }}, filter: {{ id: {{ eq: 1 }} }}) {{ id }} }}"
        );
        let response = schema.execute(Request::new(mutation)).await;
        assert_eq!(response.errors.len(), 1);
        let message = &response.errors[0].message;
        if name.chars().count() != 3 && name.chars().count() != 128 {
            assert!(message.contains("name must be between 3 and 128 characters"));
        }
        if iso6391.chars().count() != 2 || iso6391.chars().any(char::is_whitespace) {
            assert!(message.contains("iso6391 must be exactly 2 characters with no spaces"));
        }
        if iso6392.chars().count() != 3 || iso6392.chars().any(char::is_whitespace) {
            assert!(message.contains("iso6392 must be exactly 3 characters with no spaces"));
        }
    }
}

#[tokio::test]
async fn languages_query_returns_name_and_iso_codes() {
    let (_, schema) = setup().await;
    let response = schema
        .execute(Request::new(
            "{ languages { nodes { name iso6391 iso6392 } } }",
        ))
        .await;

    assert!(response.errors.is_empty(), "{:?}", response.errors);
    let data = serde_json::to_value(response.data).unwrap();
    assert_eq!(data["languages"]["nodes"][0]["name"], "English");
    assert_eq!(data["languages"]["nodes"][0]["iso6391"], "en");
    assert_eq!(data["languages"]["nodes"][0]["iso6392"], "eng");
}