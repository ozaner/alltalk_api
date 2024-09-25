use reqwest::Url;

const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:7851";

pub struct Client {
    address: Url,
}

impl Client {
    pub fn new() -> Self {
        Client::from_address(Url::parse(DEFAULT_ENDPOINT).unwrap())
    }

    pub fn from_address(address: Url) -> Self {
        Client { address }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
