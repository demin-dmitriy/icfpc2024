const API_KEY: &str = "e8de0398-92d9-41d6-9bf4-127b974095c2";

pub fn communicate(text: &str) -> String {
    use reqwest::header::AUTHORIZATION;

    let client = reqwest::blocking::Client::new();
    let url = format!("https://boundvariable.space/communicate").to_string();

    let response = client.post(url)
        .header(AUTHORIZATION, format!("Bearer {API_KEY}"))
        .body(text.to_owned())
        .send();

    response.unwrap().text().unwrap().as_str().to_string()
}
