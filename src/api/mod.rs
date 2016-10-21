use serde_json;

use hyper::header::{ Headers, Authorization };
use hyper::client::Response;
use hyper::status::StatusCode;
use hyper::Client;

use std::io::Read;

use resources::unread_counts::UnreadCounts;
use resources::{ Resource, resource_to_path };
use error::Error;

pub struct Api {
    access_token: String
}

impl Api {
    pub fn new(access_token: String) -> Self {
        Api {
            access_token: access_token
        }
    }

    pub fn unread_counts(&self) -> Result<UnreadCounts, Error> {
        let json = try!(call_api(Resource::UnreadCounts, self.access_token.as_str()));
        let unread_counts = try!(serde_json::from_str(json.as_str()));

        Ok(unread_counts)
    }
}

fn call_api(resource: Resource, access_token: &str) -> Result<String, Error> {
    let mut headers = Headers::new();
    let client = Client::new();
    let url = format!("https://cloud.feedly.com{}", resource_to_path(&resource));

    headers.set(Authorization(format!("OAuth {}", access_token).to_owned()));

    let response = try!(client.get(url.as_str()).headers(headers).send());

    match response.status {
        StatusCode::Unauthorized => Err(Error::Unauthorized),
        StatusCode::Ok => read_response(response),
        status => Err(Error::UnexpectedResponse(status))
    }
}

fn read_response(mut response: Response) -> Result<String, Error> {
    let mut buf = String::new();

    try!(response.read_to_string(&mut buf));

    Ok(buf)
}
