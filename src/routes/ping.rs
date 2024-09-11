use actix_web::get;

#[get("/")]
pub async fn rping() -> String {
    "Pong!".to_string()
}
