use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod rgba_threshold;
mod weather;

struct AppState {
    reqwest_client: reqwest::Client,
    was_raining: Mutex<bool>,
}

#[get("/weather/{x}/{y}")]
async fn get_weather(data: web::Data<AppState>, path: web::Path<(u32, u32)>) -> impl Responder {
    let (x, y) = path.into_inner();
    match weather::check_latest_weather(x, y).await {
        Ok(r) => {
            let msg = format!(
                "It is {} in ({}, {})",
                if r { "raining" } else { "not raining" },
                x,
                y
            );
            let mut wr = data.was_raining.lock().unwrap();
            if !(*wr) {
                match data
                    .reqwest_client
                    .post(format!(
                        "https://ntfy.sh/{}",
                        match std::env::var("TOPIC") {
                            Ok(var) => var,
                            Err(e) => {
                                return HttpResponse::InternalServerError().body(e.to_string());
                            }
                        }
                    ))
                    .header("Title", "Weather Alert")
                    .header("Tags", "warning")
                    .body(msg.clone()) // reqwest doesnt implement for &String so yup
                    .send()
                    .await
                {
                    Ok(_) => (),
                    Err(e) => {
                        return HttpResponse::InternalServerError().body(e.to_string());
                    }
                };
            }
            *wr = r;
            HttpResponse::Ok().body(format!(
                "Fetched {}: {}",
                weather::himawari_se3_format(),
                msg
            ))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let s = web::Data::new(AppState {
        reqwest_client: reqwest::Client::new(),
        was_raining: Mutex::new(false),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(s.clone())
            .service(echo)
            .service(get_weather)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
