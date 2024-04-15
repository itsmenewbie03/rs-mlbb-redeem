use reqwest::Client;
use serde::{Deserialize, Serialize};

trait Translate {
    fn code_to_message(code: i32) -> String;
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

#[derive(Serialize, Deserialize)]
pub struct CdkResponse {
    pub status: String,
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorCode {
    pub code: i32,
    pub lang: String,
}
impl Translate for VCodeResponse {
    fn code_to_message(code: i32) -> String {
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
}

impl Translate for CdkResponse {
    fn code_to_message(code: i32) -> String {
        let error_codes_json = include_str!("./error.json");
        let error_codes: Vec<ErrorCode> = serde_json::from_str(error_codes_json).unwrap();
        let possible_match = error_codes.iter().find(|x| x.code == code);
        match possible_match {
            None => format!("Unknown Error Code [{}]", code),
            Some(known_error_code) => known_error_code.lang.to_owned(),
        }
    }
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
    let res_mgs = VCodeResponse::code_to_message(res_body.code);
    let cdk_res = CdkResponse::code_to_message(res_body.code);
    println!("{}\n{}", res_mgs, cdk_res);
}
