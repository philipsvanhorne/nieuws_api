mod scraper;


use actix_web::{middleware, web, App, HttpServer, Responder, HttpRequest, get};
use std::{env, io};


#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info, actix_server=info");
    env_logger::init();


    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(get_news_page)
    })
        .bind("0.0.0.0:8549")?
        .run()
        .await
}




#[get("/api/news/{page}")]
async fn get_news_page(req: HttpRequest) -> impl Responder {
    let page = req.match_info().get("page").unwrap();
    let news = scraper::get_news(page).await;
    web::Json(news)
}