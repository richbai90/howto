use std::{
    env, fs,
    io::{self, Write},
};

use ::reqwest::{blocking as reqwest, Method};
use serde::Deserialize;
use serde_json::json;

pub struct Client {
    api_key: String,
    last_error: Option<String>,
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            api_key: String::new(),
            last_error: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn login(&mut self, api_key: Option<String>) {
        match get_set_key(api_key) {
            Ok(key) => self.api_key = key,
            Err(e) => {
                self.last_error = Some(e.to_string());
            }
        };
    }

    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }

    pub fn get_last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    pub fn query(&mut self, query: &str) -> Result<String, io::Error> {
        #[derive(Deserialize)]
        struct Msg {
            role: String,
            content: String,
        }

        #[derive(Deserialize)]
        struct Choice {
            message: Msg,
        }

        #[derive(Deserialize)]
        struct Response {
            choices: Vec<Choice>,
        }

        let request = self
            .client
            .request(Method::POST, "https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "gpt-3.5-turbo",
                "messages": [{"role": "user", "content": format!("Provide a command to do the following in a terminal: {}.", query)}],
                "temperature": 0.3,
            }));

        match request.send() {
            Ok(response) => {
                let body = response.json::<Response>();
                Ok(body.unwrap().choices[0].message.content.clone())
            }
            Err(e) => {
                let err = Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
                self.last_error = Some(e.to_string());
                err
            }
        }
    }
}

fn get_set_key(api_key: Option<String>) -> Result<String, io::Error> {
    let key = match api_key {
        Some(api_key) => {
            // make the directory if it doesn't exist
            let path = env::var("HOME").unwrap() + "/.config/openai";
            fs::create_dir_all(path).unwrap();
            let mut file = fs::File::create(env::var("HOME").unwrap() + "/.config/openai/api_key")?;
            file.write_all(api_key.as_bytes())?;
            api_key
        }
        None => {
            // read the api key from the file
            fs::read_to_string(env::var("HOME").unwrap() + "/.config/openai/api_key")
                .unwrap_or_default()
        }
    };

    if key.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "No API key found. Please provide one by calling auth login",
        ));
    }

    Ok(key)
}
