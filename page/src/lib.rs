use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, PanicOnDefault, AccountId};
use near_sdk::serde::{Deserialize, Serialize};

fn skeleton(title: String, inner: String) -> String {
    format!("<html lang='en'>
    <head>
      <meta charset='utf-8' />
      <title>{title}</title>
      <meta name='description' content='Find us online!' />
      <meta name='author' content='nearuaguild' />
      <meta name='viewport' content='width=device-width, initial-scale=1' />
      <link rel='stylesheet' href='css/styles.css' />
      <link rel='icon' type='image/png' href='images/avatar.png' />
    </head>
    <body>
    {inner}
    </body>
  </html>")
}

fn body(content: String) -> String {
    format!("<div class='container'>
    <div class='row'>
      <div class='column' style='margin-top: 10%'>
      {content}
      </div>
    </div>
  </div>")
}

#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
#[near_bindgen]
pub struct Contract {
    data: PageData,
    owner_id: AccountId,
    ipfs: String
}

#[derive(PanicOnDefault, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PageData {
    title: String,
    description: String,
    github: Option<String>,
    twitter: Option<String>,
    medium: Option<String>,
    telegram: Option<String>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(data: PageData, ipfs: String, owner_id: AccountId) -> Self {
        Self {
            owner_id: owner_id,
            data: data,
            ipfs
        }
    }

    pub fn update_data(&mut self, data: PageData) {
        self.assert_contract_owner();

        self.data = data;
    }

    pub fn get_data(&self) -> PageData {
        self.data.to_owned()
    }

    pub fn get_owner(&self) -> String {
        self.owner_id.to_string()
    }

    pub fn web4_get(&self, request: Web4Request) -> Web4Response {
        if request.path == "/" {

        let mut content = format!("<img
            src='images/avatar.png'
            class='avatar'
            srcset='images/avatar@2x.png 2x'
            alt='LittleLink Logo'
          />
          <h1>{title}</h1>
          <p>{description}</p>", title=self.data.title, description=self.data.description);

          if self.data.github.is_some() {
            content += format!("<a
            class='button button-github'
            href='https://github.com/{github_name}'
            target='_blank'
            rel='noopener'
          >
            <img
              class='icon'
              src='images/icons/github.svg'
              alt='GitHub Logo'
            />
            GitHub</a>
          <br />", github_name=self.data.github.as_ref().unwrap()).as_str();
          };

          if self.data.telegram.is_some() {
            content += format!("<a
            class='button button-telegram'
            href='https://t.me/{telegram_name}'
            target='_blank'
            rel='noopener'
          >
            <img
              class='icon'
              src='images/icons/telegram.svg'
              alt='Telegram Logo'
            />Telegram</a>
          <br />", telegram_name=self.data.telegram.as_ref().unwrap()).as_str();
          };

          if self.data.twitter.is_some() {
            content += format!("<a
            class='button button-twit'
            href='https://twitter.com/{twitter_name}'
            target='_blank'
            rel='noopener'
          >
            <img
              class='icon'
              src='images/icons/twitter.svg'
              alt='Twitter Logo'
            />Twitter</a>
          <br />", twitter_name=self.data.twitter.as_ref().unwrap()).as_str();
          };

          if self.data.medium.is_some() {
            content += format!("<a
            class='button button-medium'
            href='https://medium.com/{medium_name}'
            target='_blank'
            rel='noopener'
          >
            <img
              class='icon'
              src='images/icons/medium.svg'
              alt='Medium Logo'
            />Medium</a>
          <br />", medium_name=self.data.github.as_ref().unwrap()).as_str();
          };

          let body = body(content);

          let html = skeleton(self.data.title.clone(), body);

            Web4Response::Body {
                content_type: "text/html; charset=UTF-8".to_owned(),
                body: html.as_bytes().to_owned().into(),
            }
        } else
         if request.path.ends_with(".css") || request.path.ends_with(".png") || request.path.ends_with(".svg") {
            Web4Response::BodyUrl { body_url: self.ipfs.clone() + &request.path.clone() }
        } else{
            Web4Response::Status { status: 404 }
        }
    }

    fn assert_contract_owner(&self) {
        assert_condition(
            self.owner_id == env::predecessor_account_id(),
            "This method can be accessed only by the owner",
        );
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Request {
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    pub path: String,
    #[serde(default)]
    pub params: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub query: std::collections::HashMap<String, Vec<String>>,
    pub preloads: Option<std::collections::HashMap<String, Web4Response>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde", untagged)]
pub enum Web4Response {
    Body {
        #[serde(rename = "contentType")]
        content_type: String,
        body: near_sdk::json_types::Base64VecU8,
    },
    BodyUrl {
        #[serde(rename = "bodyUrl")]
        body_url: String,
    },
    PreloadUrls {
        #[serde(rename = "preloadUrls")]
        preload_urls: Vec<String>,
    },
    Status {
        status: u32
    }
}

pub fn assert_condition<S: AsRef<str>>(condition: bool, message: S) {
    if condition {
        return;
    }

    env::panic_str(message.as_ref());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}