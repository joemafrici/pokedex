use axum::{
    routing::get,
    routing::post,
    routing::put,
    extract::{Json, Query},
    Router,
};
use std::collections::HashMap;
use pokedex::models::{Pokemon, UpdatePokemon};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/add", post(handle_add_pokemon))
        .route("/api/get_all", get(handle_get_all_pokemon))
        .route("/api/get_by_name", get(handle_get_pokemon_by_name))
        .route("/api/get_by_type", get(handle_get_pokemon_by_type))
        .route("/api/edit_name", put(handle_edit_pokemon_name));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_add_pokemon(Json(pokemon): Json<Pokemon>) {
    println!("NOT IMPLEMENTED YET");
    println!("Adding pokemon: {:?}", pokemon);
}
async fn handle_get_all_pokemon() {
    println!("NOT IMPLEMENTED YET");
    println!("Getting all pokemon");
}
async fn handle_get_pokemon_by_name(Query(params): Query<HashMap<String, String>>) {
    println!("NOT IMPLEMENTED YET");
    println!("Got params: {:?}", params);
}
async fn handle_get_pokemon_by_type(Query(params): Query<HashMap<String, String>>) {
    println!("NOT IMPLEMENTED YET");
    println!("Got params: {:?}", params);
}
async fn handle_edit_pokemon_name(Json(update): Json<UpdatePokemon>) {
    println!("NOT IMPLEMENTED YET");
    println!("Updating Pokemon name. New values: {:?}", update);
}
