use diesel::prelude::*;
use std::result::Result as StdResult;

pub(crate) fn database_connection() -> StdResult<SqliteConnection, Box<std::error::Error>> {
    Ok(SqliteConnection::establish(&std::env::var(
        "DATABASE_URL",
    )?)?)
}

pub(crate) fn run_migrations() -> StdResult<(), Box<std::error::Error>> {
    //let migrations_dir = diesel_migrations::find_migrations_directory()?;
    let migrations_dir = std::path::Path::new("migrations");

    println!("cargo:rerun-if-changed={}", migrations_dir.display());

    diesel_migrations::run_pending_migrations_in_directory(
        &database_connection()?,
        &migrations_dir,
        &mut std::io::sink(),
    )?;

    Ok(())
}
