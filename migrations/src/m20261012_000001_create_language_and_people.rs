use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Languages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Languages::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Languages::Name)
                            .string_len(128)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Languages::Iso6391)
                            .string_len(2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Languages::Iso6392)
                            .string_len(3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Languages::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Languages::UpdatedAt).timestamp().null())
                    .index(
                        Index::create()
                            .name("idx_languages_iso_639_1")
                            .col(Languages::Iso6391)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("idx_languages_iso_639_2")
                            .col(Languages::Iso6392)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("idx_languages_iso_639_1_iso_639_2")
                            .col(Languages::Iso6391)
                            .col(Languages::Iso6392)
                            .unique(),
                    )
                    .check(Expr::cust("length(name) BETWEEN 3 AND 128"))
                    .check(Expr::cust(
                        "length(iso_639_1) = 2 AND instr(iso_639_1, ' ') = 0",
                    ))
                    .check(Expr::cust(
                        "length(iso_639_2) = 3 AND instr(iso_639_2, ' ') = 0",
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(People::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(People::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(People::FirstName)
                            .string_len(128)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(People::LastName)
                            .string_len(128)
                            .not_null(),
                    )
                    .col(ColumnDef::new(People::LanguageId).integer().not_null())
                    .col(ColumnDef::new(People::FavoriteColor).string().null())
                    .col(
                        ColumnDef::new(People::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(People::UpdatedAt).timestamp().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_people_language_id")
                            .from(People::Table, People::LanguageId)
                            .to(Languages::Table, Languages::Id)
                            .on_update(ForeignKeyAction::NoAction)
                            .on_delete(ForeignKeyAction::NoAction),
                    )
                            .check(Expr::cust("length(first_name) BETWEEN 3 AND 128"))
                            .check(Expr::cust("length(last_name) BETWEEN 3 AND 128"))
                    .to_owned(),
            )
            .await?;

        let connection = manager.get_connection();
        for table in ["people", "languages"] {
            connection
                .execute_unprepared(&format!(
                    "CREATE TRIGGER IF NOT EXISTS {table}_set_updated_at_on_update
                     AFTER UPDATE ON {table}
                     FOR EACH ROW
                     BEGIN
                         UPDATE {table}
                         SET updated_at = CURRENT_TIMESTAMP
                         WHERE id = NEW.id;
                     END;"
                ))
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let connection = manager.get_connection();
        for table in ["people", "languages"] {
            connection
                .execute_unprepared(&format!(
                    "DROP TRIGGER IF EXISTS {table}_set_updated_at_on_update;"
                ))
                .await?;
        }

        manager
            .drop_table(Table::drop().table(People::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Languages::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Languages {
    Table,
    Id,
    Name,
    #[sea_orm(iden = "iso_639_1")]
    Iso6391,
    #[sea_orm(iden = "iso_639_2")]
    Iso6392,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum People {
    Table,
    Id,
    FirstName,
    LastName,
    LanguageId,
    FavoriteColor,
    CreatedAt,
    UpdatedAt,
}
