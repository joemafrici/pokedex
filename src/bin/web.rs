use axum::{
    extract::{Json, Path, Query}, http::StatusCode, routing::{get, post}, Router
};
use tower_http::cors::{CorsLayer, Any};
use std::collections::HashMap;
use pokedex::models::{Pokemon, NewPokemon, DeletePokemonResponse};
use pokedex::db;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/pokemon", post(handle_add_pokemon).get(handle_get_all_pokemon))
        .route("/api/pokemon/{id}", get(handle_get_a_pokemon).put(handle_update_pokemon).delete(handle_delete_pokemon))
        .route("/api/get_by_name", get(handle_get_pokemon_by_name))
        .route("/api/get_by_type", get(handle_get_pokemon_by_type));

    let app = if cfg!(debug_assertions) {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);
        app.layer(cors)
    } else {
        app
    };

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_add_pokemon(Json(pokemon): Json<NewPokemon>) -> Result<Json<Pokemon>, StatusCode> {
    println!("Adding pokemon: {:?}", pokemon);

    let path = "data/pokedex.db";
    let conn = db::init_db(path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let saved = db::add_pokemon(&conn, pokemon).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(saved))
}
async fn handle_get_all_pokemon() -> Result<Json<Vec<Pokemon>>, StatusCode> {
    println!("Getting all pokemon");
    let path = "data/pokedex.db";

    let conn = db::init_db(path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let results = db::get_all_pokemon(&conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for pokemon in &results {
        println!("{:?}", pokemon)
    }
    Ok(Json(results))
}
async fn handle_get_a_pokemon(Path(id): Path<u32>) -> Result<Json<Pokemon>, StatusCode> {
    println!("Getting pokemon with id: {}", id);
    let path = "data/pokedex.db";
    let conn = db::init_db(path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let result = db::get_pokemon_by_id(&conn, id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match result {
        Some(p) => Ok(Json(p)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
/// Only the id in the path is used to determine which
/// record to modify
async fn handle_update_pokemon(Path(id): Path<u32>, Json(pokemon): Json<Pokemon>) -> Result<Json<Pokemon>, StatusCode> {
    println!("Updating pokemon with id: {}\n and values: {:?}", id, pokemon);

    let path = "data/pokedex.db";
    let conn = db::init_db(path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let result = db::edit_pokemon_by_id(&conn, id, pokemon).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match result {
        Some(p) => Ok(Json(p)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
async fn handle_delete_pokemon(Path(id): Path<u32>) -> Result<Json<DeletePokemonResponse>, StatusCode> {
    println!("Deleting pokemon with id: {}", id);

    let path = "data/pokedex.db";
    let conn = db::init_db(path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let result = db::delete_pokemon_by_id(&conn, id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(DeletePokemonResponse {
        deleted: result,
    }))
}
async fn handle_get_pokemon_by_name(Query(params): Query<HashMap<String, String>>) {
    println!("NOT IMPLEMENTED YET");
    println!("Got params: {:?}", params);
}
async fn handle_get_pokemon_by_type(Query(params): Query<HashMap<String, String>>) {
    println!("NOT IMPLEMENTED YET");
    println!("Got params: {:?}", params);
}
