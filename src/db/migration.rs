use entity::*;
use sea_orm::{ConnectionTrait, DbConn, EntityTrait, Schema};

async fn create_table<E>(db: &DbConn, entity: E)
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    // let tcs = &schema.create_table_from_entity(entity);
    // let stmt = builder.build(tcs);
    let stmt = builder.build(Schema::create_table_from_entity(&schema, entity).if_not_exists());

    match db.execute(stmt).await {
        Ok(_) => println!("Migrated {}", entity.table_name()),
        Err(e) => println!("Error: {}", e),
    }
}

pub async fn create_tables(db: &DbConn) {
    create_table(db, Role).await;
    create_table(db, Address).await;
    create_table(db, User).await;
    create_table(db, UsrAssoRol).await;
    create_table(db, UsrAssoAdr).await;
}