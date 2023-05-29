use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct News {
    pub title: String,
    pub href: String,
    pub image: String,
    pub page_url: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct NewsDetails {
    pub title: String,
    pub date: String,
    pub content: String,
    pub images: Vec<String>,
    pub links: Vec<String>,
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
        let page_url = href.split("/").collect::<Vec<_>>()[4].to_string();

        news.push(News {
            title,
            href: format!("https://pvanhorne.nl{}", href),
            image,
            page_url,
        });
    }

    news
}


pub async fn get_news_details(page_url: &str) -> NewsDetails {
    let response = reqwest::get(format!("https://pvanhorne.nl/leerlingen/overzichten/nieuws-1/{}", page_url))
                    .await.unwrap()
                    .text()
                    .await.unwrap();
    let document = scraper::Html::parse_document(&response);
    let selector = scraper::Selector::parse("div[class='news-view']").unwrap();

    let title = document.select(&scraper::Selector::parse("h1").unwrap())
                    .next().unwrap().text()
                    .collect::<Vec<_>>()
                    .join("").trim().to_string();

    let date = document.select(&scraper::Selector::parse("div[class='news-date']").unwrap())
                    .next().unwrap().text()
                    .collect::<Vec<_>>()
                    .join("").trim().to_string();

    let images = document.select(&selector)
                    .next().unwrap()
                    .select(&scraper::Selector::parse("img").unwrap())
                    .map(|element| format!("https://pvanhorne.nl{}", element.value().attr("src").unwrap()))
                    .collect::<Vec<String>>();

    let links = document.select(&selector)
                    .next().unwrap()
                    .select(&scraper::Selector::parse("a:not([class='news-back'])").unwrap())
                    .map(|element| {
                        let href = element.value().attr("href").unwrap();
                        if href.starts_with("http") { href.to_string() }
                        else { format!("https://pvanhorne.nl{}", href) }
                    })
                    .collect::<Vec<String>>();

    let content_selector = scraper::Selector::parse("div[class='news-view-content']").unwrap();
    let mut content: Vec<String> = Vec::new();

    for element in document.select(&content_selector) {
        content.push(
            element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .replace("\n", "")
                .replace("\t", "")
                .replace("\r", "")
                .replace("  ", "")
                .trim().to_string()
        );
    }


    NewsDetails {
        title,
        images,
        date: parse_date(&date).await,
        content: content.join(" "),
        links,
    }
}


async fn parse_date(date: &str) -> String {
    let date = date.split(" ").collect::<Vec<_>>();
    let day = date[0];
    let month = match date[1] {
        "januari" => "01",
        "februari" => "02",
        "maart" => "03",
        "april" => "04",
        "mei" => "05",
        "juni" => "06",
        "juli" => "07",
        "augustus" => "08",
        "september" => "09",
        "oktober" => "10",
        "november" => "11",
        "december" => "12",
        _ => "00",
    };
    let year = date[2];

    format!("{}-{}-{}", day, month, year)
}