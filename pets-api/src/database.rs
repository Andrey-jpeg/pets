use sea_orm::*;

pub async fn init_db(connection_url: String) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(connection_url).await?;
    Ok(db)
}