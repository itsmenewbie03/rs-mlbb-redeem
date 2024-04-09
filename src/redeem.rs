use reqwest::Client;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VCodeBody {
    pub roleId: String,
    pub zoneId: String,
    pub captcha: String,
    pub language: String,
}

pub async fn send_vc(game_id: &str, server_id: &str) {
    let cli = Client::new();
    let url = "https://api.mobilelegends.com/mlweb/sendMail";
    let body = VCodeBody {
        roleId: game_id.to_owned(),
        zoneId: server_id.to_owned(),
        captcha: "".to_owned(),
        language: "en".to_owned(),
    };
    let res = cli.post(url).json(&body).send().await.unwrap();
    let res_body = res.text().await.unwrap();
    println!(":: RESPONSE: {}", res_body);
}
