use reqwest::Client;
use serde::{Deserialize, Serialize};
// { "-20023": "Invalid Game ID", "-20027": "Request too Frequent!...", "-20028": "Verification code already sent...", "-20010": "Invalid Verification Code!", "0": "Verification Code Sent Successfully!", "1401": "redeem in specified zone", "1402": "This CDKey does not exist", "1403": "CDKey expired", "1404": "Incorrect format of CDKey", "1405": "This CDKey has been redeemed.", "1406": "Bound Account CDKey. Incorrect account.", "1407": "Exceeds exchange amount limit.", "1408": "Can only redeem in specified zone.", "1409": "Restriction Requirement Configuration Error", "1410": "This CDKey is being redeemed by many players. The Server is processing... Please try again later.", "1411": "It\'s not exchange time, please wait.", "1412": "Limit reached for number of people exchanging.", "1413": "You are not a new user", "1414": "You haven\'t purchased yet", "1415": "Your level is too high", "1416": "You can not redeem the CDKey through your channel", "1036": "The amount limitation of CDKey redeemption" }

pub fn code_to_message(code: i32) -> String {
    match code {
        -20023 => "Invalid Game ID".to_owned(),
        -20027 => "Request too Frequent!".to_owned(),
        -20028 => "Verification code already sent".to_owned(),
        -20010 => "Invalid Verification Code!".to_owned(),
        -20025 => "Captcha Verification Required".to_owned(),
        0 => "Verification Code Sent Successfully!".to_owned(),
        1401 => "redeem in specified zone".to_owned(),
        1402 => "This CDKey does not exist".to_owned(),
        1403 => "CDKey expired".to_owned(),
        1404 => "Incorrect format of CDKey".to_owned(),
        1405 => "This CDKey has been redeemed.".to_owned(),
        1406 => "Bound Account CDKey. Incorrect account.".to_owned(),
        1407 => "Exceeds exchange amount limit.".to_owned(),
        1408 => "Can only redeem in specified zone.".to_owned(),
        1409 => "Restriction Requirement Configuration Error".to_owned(),
        1410 => "This CDKey is being redeemed by many players. The Server is processing... Please try again later.".to_owned(),
        1411 => "It's not exchange time, please wait.".to_owned(),
        1412 => "Limit reached for number of people exchanging.".to_owned(),
        1413 => "You are not a new user".to_owned(),
        1414 => "You haven't purchased yet".to_owned(),
        1415 => "Your level is too high".to_owned(),
        1416 => "You can not redeem the CDKey through your channel".to_owned(),
        1036 => "The amount limitation of CDKey redeemption".to_owned(),
        _ => "Unknown Error".to_owned(),
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct VCodeBody {
    pub roleId: String,
    pub zoneId: String,
    pub captcha: String,
    pub language: String,
}

#[derive(Serialize, Deserialize)]
pub struct VCodeResponse {
    pub status: String,
    pub code: i32,
    pub message: String,
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
    let res_body = res.json::<VCodeResponse>().await.unwrap();
    let res_mgs = code_to_message(res_body.code);
    println!("{}", res_mgs);
}
