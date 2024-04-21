use sea_orm_migration::{prelude::*, sea_orm::Schema};
use entity::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let schema = Schema::new(manager.get_database_backend());

        manager.create_table(Schema::create_table_from_entity(&schema, Address)).await?;
        manager.create_table(Schema::create_table_from_entity(&schema, Role)).await?;
        manager.create_table(Schema::create_table_from_entity(&schema, User)).await?;
        manager.create_table(Schema::create_table_from_entity(&schema, UsrAssoRol)).await?;
        manager.create_table(Schema::create_table_from_entity(&schema, UsrAssoAdr)).await?;

        let insert_role = Query::insert()
            .into_table(Role)
            .columns([entity::role::Column::Libelle])
            .values_panic(vec!["Admin".into()])
            .values_panic(vec!["Compta".into()])
            .values_panic(vec!["Livreur".into()])
            .values_panic(vec!["Client".into()])
            .to_owned();
        
        manager.exec_stmt(insert_role).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(entity::UsrAssoAdr).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(entity::UsrAssoRol).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(entity::User).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(entity::Role).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(entity::Address).to_owned())
            .await?;

        Ok(())
    }
}
