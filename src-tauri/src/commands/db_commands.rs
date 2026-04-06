use tauri::AppHandle;

use crate::{
    db,
    errors::AppResult,
    models::DatabaseStatus,
};

#[tauri::command]
pub async fn database_status(app: AppHandle) -> AppResult<DatabaseStatus> {
    let database_path = db::database_path(&app)?
        .display()
        .to_string();
    let _pool = db::connect(&app).await?;

    Ok(DatabaseStatus {
        ready: true,
        database_path,
    })
}

