use std::path::PathBuf;

use mongodb::Database;

use crate::errors::ServiceError;

mod exit;
mod navigation;
mod snapshot_creation;

pub async fn run_command(
    input: i32,
    path: &mut PathBuf,
    db: &Database,
) -> Result<(), ServiceError> {
    match input {
        1 => {
            navigation::nav_menu(path);
        }
        2 => {
            snapshot_creation::create_snapshot(path, db).await?;
        }
        5 => {
            exit::exit();
        }
        _ => {
            println!("Incorrect input. Please repeat your attempt");
        }
    };

    Ok(())
}
