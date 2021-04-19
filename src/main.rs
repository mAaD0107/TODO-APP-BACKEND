
mod config;
mod model;
mod controller; 
mod dao; 

use warp::Filter;
use actix_cors::Cors;
use actix_web::{HttpServer,App, web};
use actix_web::http::header;
use std::io; 
use dotenv::dotenv; 
use tokio_postgres::NoTls;
use crate::controller::*; 


#[actix_rt::main]
async fn main() -> io::Result<()>{
    
    dotenv().ok(); 
    //Cors::new().supports_credentials();
    
    //let cors = Cors::permissive();

    let cors = warp::cors()
    .allow_any_origin()
    .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers"])
    .allow_methods(vec!["POST", "GET"]);

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Iniciando Servidor en http//{}:{}/", config.server.host, config.server.port);

    HttpServer::new(move ||{
        App::new()
        .data(pool.clone())
        .route("/", web::get().to(status))
        .route("/todos{_:/?}", web::get().to(get_todos))
        .route("/todos{_:/?}", web::post().to(create_todo))
        .route("/todos/{list_id}/items{_:/?}", web::get().to(get_items))
        .route("/todos/{list_id}/items/{item_id}{_:/?}", web::put().to(check_item))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
