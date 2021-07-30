#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
    use sqlx::migrate::Migrator;
    use sqlx::postgres::PgPoolOptions;
    use std::path::Path;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/postgres")
        .await
        .expect("Should have created the pool");

    let m = Migrator::new(Path::new("./migrations"))
        .await
        .expect("Should have created migrator");
    m.run(&pool).await.expect("Failed to run migrations");
    println!("Hello, world!");
    Ok(())
}
