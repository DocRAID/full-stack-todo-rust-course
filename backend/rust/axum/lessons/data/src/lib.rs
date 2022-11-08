mod routes;

use sea_orm::Database;

pub async fn run(database_uri: &str) {
    let _database = Database::connect(database_uri).await;
    let app = routes::create_routes().await;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
