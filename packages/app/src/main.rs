mod routes;

use crate::routes::{delete_game, get_details, get_game, get_games, post_move, put_game};
use actix_web::http::header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE};
use actix_web::http::Method;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use core::game_manager::GameManager;
use std::sync::Mutex;

struct AppState {
    game_manager: Mutex<GameManager>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ADDRESS: (&str, u16) = ("127.0.0.1", 8080);

    let state = web::Data::new(AppState {
        game_manager: Mutex::new(GameManager::new()),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::DefaultHeaders::new().add((CONTENT_TYPE, "application/json; charset=utf-8")))
            .wrap(middleware::DefaultHeaders::new().add((ACCESS_CONTROL_ALLOW_ORIGIN, "*")))
            .wrap(middleware::DefaultHeaders::new().add((ACCESS_CONTROL_ALLOW_METHODS, "*")))
            .wrap(middleware::DefaultHeaders::new().add((ACCESS_CONTROL_ALLOW_HEADERS, "*")))
            .service(get_details)
            .service(get_games)
            .service(put_game)
            .service(get_game)
            .service(delete_game)
            .service(post_move)
            .default_service(web::route().method(Method::OPTIONS).to(HttpResponse::Ok))
    }).bind(ADDRESS)?
        .run();

    println!("Server running at http://{}:{}/", ADDRESS.0, ADDRESS.1);
    server.await
}
