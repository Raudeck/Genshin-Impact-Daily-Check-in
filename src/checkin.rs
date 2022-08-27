use std::borrow::Borrow;

use serde::{Deserialize, Serialize};
use reqwest::{self, header::{COOKIE, USER_AGENT, HeaderMap, HeaderValue, ORIGIN, REFERER}};

pub struct AccountInfo {
    cookies: String
}

#[derive(Deserialize, Serialize, Debug)]
struct ParseJSON {
    message: String
}

impl AccountInfo {
    #[tokio::main]
    pub async fn check_in(&self) -> reqwest::Result<()>{
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(COOKIE, HeaderValue::from_str(self.cookies.borrow()).unwrap());
        headers.insert(ORIGIN, HeaderValue::from_str("https://webstatic-sea.mihoyo.com").unwrap());
        headers.insert(REFERER, HeaderValue::from_str("https://webstatic-sea.mihoyo.com/").unwrap());
        headers.insert(USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.102 Safari/537.36 Edg/104.0.1293.63").unwrap());
        
        let resp = client.post("https://hk4e-api-os.mihoyo.com/event/sol/sign?lang=en-us&act_id=e202102251931481")
            .headers(headers)
            .send()
            .await?
            .json::<ParseJSON>()
            .await?
        ;

        println!("{:#?}", resp);

        Ok(())
    }

    pub fn new(input: String) -> AccountInfo {
        AccountInfo { cookies: (input) }
    }
}