use reqwasm::http::Request;

pub async fn get_beers() -> String {
    let url = "http://localhost:8080/";

    Request::get(&url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
