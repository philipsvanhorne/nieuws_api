use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct News {
    pub title: String,
    pub href: String,
    pub image: String,
}


pub async fn get_news(page_index: &str) -> Vec<News> {
    let response = reqwest::get(format!("https://pvanhorne.nl/leerlingen/overzichten/nieuws-1/p:{}", page_index)).await.unwrap().text().await.unwrap();
    let document = scraper::Html::parse_document(&response);
    let selector = scraper::Selector::parse("a[class='news-item']").unwrap();
    let mut news: Vec<News> = Vec::new();


    for element in document.select(&selector) {
        let title = element.select(&scraper::Selector::parse("h3").unwrap()).next().unwrap().inner_html();
        let href = element.value().attr("href").unwrap().to_string();
        let image = match element.select(&scraper::Selector::parse("img").unwrap()).next() {
            Some(image) => format!("https://pvanhorne.nl{}", image.value().attr("src").unwrap()),
            None => String::from(""),
        };

        news.push(News {
            title,
            href: format!("https://pvanhorne.nl{}", href),
            image,
        });
    }

    news
}