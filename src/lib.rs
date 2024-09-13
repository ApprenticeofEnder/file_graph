use std::fs::File;
use std::io::Write;

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::web::Html;
use actix_web::{get, middleware, route, web, App, HttpResponse, HttpServer, Responder};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

mod routes;
use crate::routes::ping::rping;
pub mod schemas;
use crate::schemas::schema::{create_schema, Schema};

/// Playground
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html::new(graphiql_source("/graphql", None))
}

/// Endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

pub fn init_logger() -> () {
    let env = env_logger::Env::new().default_filter_or("info");
    env_logger::init_from_env(env);
}

pub fn init_server() -> std::io::Result<Server> {
    let schema = std::sync::Arc::new(create_schema());

    let sdl = schema.as_sdl();

    let mut file = File::create("tests/graphql/schema.graphql")?;
    write!(file, "{}", sdl).unwrap();

    let port = 8080;

    log::info!("Starting on Port: http://localhost:{}", port);
    log::info!("Playground running on: http://localhost:{}/graphiql", port);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .service(rping)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .bind(("0.0.0.0", port))?
    .run();

    Ok(server)
}
