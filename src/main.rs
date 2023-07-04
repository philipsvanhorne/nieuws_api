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
            .service(get_news_details)
            .service(get_photo_books)
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

#[get("/api/news/details/{page_url}")]
async fn get_news_details(req: HttpRequest) -> impl Responder {
    let page_url = req.match_info().get("page_url").unwrap();
    let news_details = scraper::get_news_details(page_url).await;
    web::Json(news_details)
}

#[get("/api/photobooks/{year}")]
async fn get_photo_books(req: HttpRequest) -> impl Responder {
    let year = req.match_info().get("year").unwrap();
    let photo_books = scraper::get_photo_books(year).await;
    web::Json(photo_books)
}