use crate::{entities::*, guards::{validate_language, validate_person}};
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static, Builder, BuilderContext};

lazy_static::lazy_static! {
    static ref CONTEXT: BuilderContext = {
        let mut context = BuilderContext::default();

        // note: use table names
        for entity in ["People", "Languages"] {
            for field in ["id", "createdAt", "updatedAt"] {
                let field_name = format!("{entity}.{field}");
                context.entity_input.insert_skips.push(field_name.clone());
                context.entity_input.update_skips.push(field_name);
            }
        }

        context.guards.entity_guards.insert(
            "Languages".into(),
            Box::new(validate_language),
        );
        context.guards.entity_guards.insert(
            "People".into(),
            Box::new(validate_person),
        );

        context
    };
}

pub fn schema(
    database: DatabaseConnection,
    depth: usize,
    complexity: usize,
) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    seaography::register_entities!(builder, [language, person,]);

    builder
        .set_depth_limit(Some(depth))
        .set_complexity_limit(Some(complexity))
        .schema_builder()
        .data(database)
        .finish()
}
