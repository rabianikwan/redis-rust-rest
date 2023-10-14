use mongodb::Database;
use tokio::sync::OnceCell;
use crate::config::config::SETTING;

static CONNECTION: OnceCell<Database> = OnceCell::const_new();

pub async fn connection() -> &'static Database {
    CONNECTION
        .get_or_init(|| async {
            let db_uri = SETTING.database.uri.as_str();
            let db_name = SETTING.database.name.as_str();

            mongodb::Client::with_uri_str(db_uri)
                .await
                .expect("Fail to initialize MongoDB Connection")
                .database(db_name)
        })
        .await
}