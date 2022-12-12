use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod rgba_threshold;
mod weather;

#[get("/weather/{x}/{y}")]
async fn get_weather(path: web::Path<(u32, u32)>) -> impl Responder {
    let (x, y) = path.into_inner();
    match weather::check_latest_weather(x, y).await {
        Ok(r) => HttpResponse::Ok().body(format!(
            "It is {} in ({}, {})",
            if r { "raining" } else { "not raining" },
            x,
            y
        )),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(echo).service(get_weather))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
