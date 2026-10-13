use crate::entities::favorite_color::FavoriteColor;
use seaography::{async_graphql::dynamic::ResolverContext, GuardAction};
use std::str::FromStr;

fn is_valid_favorite_color(input: &str) -> bool {
    FavoriteColor::from_str(input).is_ok()
}

pub fn validate_person(ctx: &ResolverContext<'_>) -> GuardAction {
    let Some(input) = ctx.args.get("data") else {
        return GuardAction::Allow;
    };

    let Ok(item) = input.object() else {
        return GuardAction::Block(Some("data must be an object".into()));
    };

    let mut errors = Vec::new();

    for field_name in ["firstName", "lastName"] {
        if let Some(value) = item.get(field_name) {
            if !value.is_null() {
                match value.string() {
                    Ok(name) if !(3..=128).contains(&name.chars().count()) => {
                        errors.push(format!(
                            "{field_name} must be between 3 and 128 characters"
                        ));
                    }
                    Err(_) => errors.push(format!("{field_name} must be a string")),
                    _ => {}
                }
            }
        }
    }

    if item.get("favoriteColor").is_some_and(|value| {
        if value.is_null() {
            false
        } else {
            value
                .string()
                .map_or(true, |color| !is_valid_favorite_color(color))
        }
    }) {
        errors.push(format!(
            "favoriteColor must be one of: {}",
            FavoriteColor::allowed_values()
        ));
    }

    if errors.is_empty() {
        GuardAction::Allow
    } else {
        GuardAction::Block(Some(errors.join("; ")))
    }
}