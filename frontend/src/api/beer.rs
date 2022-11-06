use reqwasm::http::Request;

use super::get_api_url;

pub async fn get_beers() -> String {
    let url = get_api_url("");

    Request::get(&url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
