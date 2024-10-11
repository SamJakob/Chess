use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use core::game::Game;
use core::moves::Position;
use serde::Serialize;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
struct VersionResponse<'a> {
    version: &'a str,
}

#[get("/details")]
async fn get_details() -> impl Responder {
    HttpResponse::Ok().body(serde_json::to_string(&VersionResponse { version: VERSION }).unwrap())
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
        Ok((_, game)) => HttpResponse::Ok().body(serde_json::to_string(&game).unwrap()),
        Err(e) => e,
    }
}

#[delete("/game/{id}")]
async fn delete_game(data: web::Data<AppState>, game_id: web::Path<String>) -> impl Responder {
    let mut game_manager = data.game_manager.lock().unwrap();
    match locate_game_by_id(data.clone(), game_id.into_inner()) {
        Ok((id, _)) => {
            game_manager.delete_game(id);
            HttpResponse::Ok().finish()
        }
        Err(e) => e,
    }
}

#[post("/game/{game_id}/{position}/move")]
async fn post_move(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
    new_position: web::Json<Position>,
) -> impl Responder {
    let (game_id, raw_position) = path.into_inner();
    let position = serde_json::from_str::<Position>(&raw_position);

    // Ensure the position in the URL is valid.
    if let Err(e) = position {
        return HttpResponse::BadRequest().body(format!("{:?}", e));
    }

    let position = position.unwrap();
    let new_position = new_position.into_inner();

    match locate_game_by_id(data, game_id) {
        Ok((_, game)) => {
            match game
                .lock()
                .unwrap()
                .move_piece_at_position(&position, &new_position)
            {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(e) => HttpResponse::NotFound().body(format!("{:?}", e)),
            }
        }
        Err(e) => e,
    }
}

fn locate_game_by_id(
    data: web::Data<AppState>,
    id: String,
) -> Result<(Uuid, Arc<Mutex<Game>>), HttpResponse> {
    let uuid = Uuid::from_str(id.as_str());

    match uuid {
        Ok(uuid) => {
            let game_manager = data.game_manager.lock().unwrap();
            match game_manager.get_game(uuid) {
                Some(game) => Ok((uuid, game)),
                None => {
                    Err(HttpResponse::NotFound().body("No game found for the supplied game ID"))
                }
            }
        }
        Err(_) => Err(HttpResponse::BadRequest().body("Invalid game ID")),
    }
}
