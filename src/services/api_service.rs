use std::error::Error;

use serde::de::DeserializeOwned;

pub trait ApiService: Send {
    async fn get_json_data<Response>(
        &self,
        url: impl Into<String>,
    ) -> Result<Response, Box<dyn Error>>
    where
        Response: DeserializeOwned;
}

impl ApiService for reqwest::Client {
    async fn get_json_data<Response: DeserializeOwned>(
        &self,
        url: impl Into<String>,
    ) -> Result<Response, Box<dyn Error>> {
        let url = url.into();
        match self.get(url.clone()).send().await {
            Ok(response) => match response.json::<Response>().await {
                Ok(json) => Ok(json),
                Err(e) => {
                    eprintln!("Call to this API url failed: {}", url);
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                eprintln!("Call to this API url failed: {}", url);
                Err(Box::new(e))
            }
        }
    }
}

impl ApiService for reqwest_middleware::ClientWithMiddleware {
    async fn get_json_data<Response: DeserializeOwned>(
        &self,
        url: impl Into<String>,
    ) -> Result<Response, Box<dyn Error>> {
        let url = url.into();
        match self.get(url.clone()).send().await {
            Ok(response) => match response.json::<Response>().await {
                Ok(json) => Ok(json),
                Err(e) => {
                    println!("Call to this API url failed: {}", url);
                    Err(Box::new(e))
                }
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}
