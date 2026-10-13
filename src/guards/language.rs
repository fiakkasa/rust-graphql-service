use seaography::{async_graphql::dynamic::ResolverContext, GuardAction};

pub fn validate_language(ctx: &ResolverContext<'_>) -> GuardAction {
    let Some(input) = ctx.args.get("data") else {
        return GuardAction::Allow;
    };

    let Ok(item) = input.object() else {
        return GuardAction::Block(Some("data must be an object".into()));
    };

    let mut errors = Vec::new();

    if let Some(value) = item.get("name") {
        if !value.is_null() {
            match value.string() {
                Ok(name) if !(3..=128).contains(&name.chars().count()) => {
                    errors.push("name must be between 3 and 128 characters".to_string());
                }
                Err(_) => errors.push("name must be a string".to_string()),
                _ => {}
            }
        }
    }

    for (field_name, expected_length) in [("iso6391", 2), ("iso6392", 3)] {
        if let Some(value) = item.get(field_name) {
            if !value.is_null() {
                match value.string() {
                    Ok(code)
                        if code.chars().count() != expected_length
                            || code.chars().any(char::is_whitespace) =>
                    {
                        errors.push(format!(
                            "{field_name} must be exactly {expected_length} characters with no spaces"
                        ));
                    }
                    Err(_) => errors.push(format!("{field_name} must be a string")),
                    _ => {}
                }
            }
        }
    }

    if errors.is_empty() {
        GuardAction::Allow
    } else {
        GuardAction::Block(Some(errors.join("; ")))
    }
}