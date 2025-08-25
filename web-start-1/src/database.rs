use super::config;
use mongodb::{Client, options::ClientOptions};
use sea_orm::DatabaseConnection;

pub async fn init() -> anyhow::Result<(), mongodb::error::Error> {
    // 连接 MongoDB（默认端口 27017）
    let database_config = &config::get().database();
    // 需要输出 database_config 的内容
    println!("{:?}", database_config);
    Ok(())
}

//
async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    // let version_result = db
    //     .query_one(Statement::from_string(
    //         Postgres,
    //         String::from("Select version ()"),
    //     ))
    //     .await?
    //     .ok_or_else(|| anyhow::anyhow!("Database version not found"));
    // tracing::info!(
    //     "Database version: {}",
    //     version_result?.try_get_by_index::<String>(0)?
    // );
    Ok(())
}
