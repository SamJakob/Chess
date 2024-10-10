use crate::AppState;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use core::game::Game;
use core::moves::Position;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[get("/version")]
async fn get_version() -> impl Responder {
    HttpResponse::Ok().body(VERSION)
}

#[get("/games")]
async fn get_games(data: web::Data<AppState>) -> impl Responder {
    let game_manager = data.game_manager.lock().unwrap();
    let games = game_manager.get_all_games();
    HttpResponse::Ok().body(serde_json::to_string(&games).unwrap())
}

#[put("/game")]
async fn put_game(data: web::Data<AppState>) -> impl Responder {
    let mut game_manager = data.game_manager.lock().unwrap();
    let game = game_manager.new_game();
    HttpResponse::Ok().body(serde_json::to_string(&game).unwrap())
}

#[get("/game/{id}")]
async fn get_game(data: web::Data<AppState>, game_id: web::Path<String>) -> impl Responder {
    match locate_game_by_id(data, game_id.into_inner()) {
        Ok(game) => HttpResponse::Ok().body(serde_json::to_string(&game).unwrap()),
        Err(e) => e
    }
}

#[post("/game/{game_id}/{position}/move")]
async fn post_move(path: web::Path<(String, String)>, _new_position: web::Json<Position>) -> impl Responder {
    let (_game_id, _position) = path.into_inner();

    HttpResponse::Ok()
}

fn locate_game_by_id(data: web::Data<AppState>, id: String) -> Result<Arc<Game>, HttpResponse> {
    let uuid = Uuid::from_str(id.as_str());

    match uuid {
        Ok(uuid) => {
            let game_manager = data.game_manager.lock().unwrap();
            match game_manager.get_game(uuid) {
                Some(game) => Ok(game),
                None => Err(HttpResponse::NotFound().body("No game found for the supplied game ID")),
            }
        }
        Err(_) => Err(HttpResponse::BadRequest().body("Invalid game ID")),
    }
}
