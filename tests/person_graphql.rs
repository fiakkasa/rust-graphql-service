mod common;

use common::{setup, updated_at};
use seaography::async_graphql::Request;

#[tokio::test]
async fn people_update_refreshes_updated_at() {
    let (database, schema) = setup().await;
    let response = schema
        .execute(Request::new(
            "mutation { peopleUpdate(data: { firstName: \"Augusta\" }, filter: { id: { eq: 1 } }) { id } }",
        ))
        .await;

    assert!(response.errors.is_empty(), "{:?}", response.errors);
    let timestamp = updated_at(&database, "people").await;
    assert!(timestamp.is_some());
    assert_ne!(timestamp.as_deref(), Some("2000-01-01 00:00:00"));
}

#[tokio::test]
async fn people_mutations_validate_names_and_favorite_color() {
    let (_, schema) = setup().await;

    for color in ["Red", "Blue", "Green"] {
        let mutation = format!(
            "mutation {{ peopleUpdate(data: {{ favoriteColor: \"{color}\" }}, filter: {{ id: {{ eq: 1 }} }}) {{ id }} }}"
        );
        let response = schema.execute(Request::new(mutation)).await;
        assert!(response.errors.is_empty(), "{:?}", response.errors);
    }

    for field_name in ["firstName", "lastName"] {
        for name in ["Ada".to_string(), "A".repeat(128)] {
            let mutation = format!(
                "mutation {{ peopleUpdate(data: {{ {field_name}: \"{name}\" }}, filter: {{ id: {{ eq: 1 }} }}) {{ id }} }}"
            );
            let response = schema.execute(Request::new(mutation)).await;
            assert!(
                response.errors.is_empty(),
                "valid {field_name} length {} was rejected: {:?}",
                name.chars().count(),
                response.errors
            );
        }
    }

    let response = schema
        .execute(Request::new(
            "mutation { peopleUpdate(data: { firstName: \"Al\", lastName: \"Li\", favoriteColor: \"Purple\" }, filter: { id: { eq: 1 } }) { id } }",
        ))
        .await;
    assert_eq!(response.errors.len(), 1);
    for message in [
        "firstName must be between 3 and 128 characters",
        "lastName must be between 3 and 128 characters",
        "favoriteColor must be one of: Red, Blue, Green",
    ] {
        assert!(response.errors[0].message.contains(message));
    }

    for field_name in ["firstName", "lastName"] {
        let too_long_name = "A".repeat(129);
        let mutation = format!(
            "mutation {{ peopleUpdate(data: {{ {field_name}: \"{too_long_name}\" }}, filter: {{ id: {{ eq: 1 }} }}) {{ id }} }}"
        );
        let response = schema.execute(Request::new(mutation)).await;
        assert_eq!(response.errors.len(), 1);
        assert!(response.errors[0]
            .message
            .contains(&format!("{field_name} must be between 3 and 128 characters")));
    }

    let response = schema
        .execute(Request::new(
            "mutation { peopleCreateOne(data: { firstName: \"Alan\", lastName: \"Turing\", languageId: 1, favoriteColor: \"Purple\" }) { id } }",
        ))
        .await;
    assert_eq!(response.errors.len(), 1);
    assert!(response.errors[0]
        .message
        .contains("favoriteColor must be one of: Red, Blue, Green"));
}

#[tokio::test]
async fn people_query_returns_favorite_color() {
    let (_, schema) = setup().await;
    let response = schema
        .execute(Request::new("{ people { nodes { favoriteColor } } }"))
        .await;

    assert!(response.errors.is_empty(), "{:?}", response.errors);
    let data = serde_json::to_value(response.data).unwrap();
    assert_eq!(data["people"]["nodes"][0]["favoriteColor"], "Red");
}