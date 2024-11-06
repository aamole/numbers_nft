use reqwest::{self, header};
use scraper::{Html, Selector};
use regex::Regex;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    
    let response = client.get("https://fragment.com/numbers?sort=ending")
        .header(header::USER_AGENT, "Mozilla/5.0")
        .send()
        .await?;
    
    let response_t: String = response.text().await?;

    let row_s = Selector::parse("tr.tm-row-selectable").unwrap();

    for row in Html::parse_document(&response_t).select(&row_s) {

        if let Some(number_e) = row.select(&Selector::parse("div.tm-value").unwrap()).next() {
            let number = number_e.text().collect::<String>().trim().to_string();
            let resale_status = row.select(&Selector::parse("div.table-cell-status-thin").unwrap()).next()
                .map(|elem| elem.text().collect::<String>().trim().to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            
            let bid_price_str = row.select(&Selector::parse("div.icon-ton").unwrap()).next()
                .map(|elem| elem.text().collect::<String>().trim().replace(",", ""))
                .unwrap_or_else(|| "0".to_string());
            
            let bid_price: i32 = bid_price_str.parse().unwrap_or(0);

            let remaining_time = row.select(&Selector::parse("time").unwrap()).next()
                .map(|elem| elem.text().collect::<String>().trim().to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            let days: i32 = Regex::new(r"(\d+)\s+days")?.captures(&remaining_time)
                .and_then(|caps| caps[1].parse::<i32>().ok())
                .unwrap_or(100000);

            if bid_price < 200 && days < 7 && (remaining_time.contains("hours") || remaining_time.contains("days")) {
                println!("Number: https://fragment.com/number/{}", number.replace("+", "").replace(" ", ""));
                println!("Resale Status: {}", resale_status);
                println!("Bid Price: {}", bid_price_str);
                println!("Auction End Date: {}", remaining_time);
                println!();
            }
        }
    }
    Ok(())
}
