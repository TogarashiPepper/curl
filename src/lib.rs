use reqwest::{RequestBuilder, Client};
use std::process::exit;
use log::error;
use reqwest::Url;
pub trait ArgHandler {
    fn optional_body(self, body: Option<String>) -> RequestBuilder;
}

impl ArgHandler for RequestBuilder {
    fn optional_body(self, body: Option<String>) -> RequestBuilder {
        match body {
            Some(i) => {
                println!("{}", &i);
                self.body(i)
            },
            _ => self
        }
    }
}

pub fn create_builder_request(url: &str, method: &str, client: Client, port:Option<u16>) -> RequestBuilder {
    let mut url = Url::parse(url).unwrap_or_else(|_| {
        error!("Error parsing URL");
        exit(1);
    });
    url.set_port(port).map_err(|_| "cannot be base").unwrap();

    match method {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "HEAD" => client.head(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        _ => {
            error!("Unknown Method");
            exit(1);
        }
    }
}