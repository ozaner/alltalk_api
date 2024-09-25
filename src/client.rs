use reqwest::Url;
use serde::Deserialize;

const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:7851";

pub struct Client {
    address: Url,
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Client::from_address(Url::parse(DEFAULT_ENDPOINT).unwrap())
    }

    pub fn from_address(address: Url) -> Self {
        Client {
            address,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_ready(&self) -> reqwest::Result<bool> {
        let res = self
            .client
            .get(self.address.join("api/ready").unwrap())
            .send()
            .await?;

        Ok(res.text().await? == "Ready") //should otherwise be "Unloaded" (anything else is invalid)
    }

    pub async fn get_voices_list(&self) -> reqwest::Result<Vec<String>> {
        let res = self
            .client
            .get(self.address.join("api/voices").unwrap())
            .send()
            .await?;

        #[derive(Deserialize)]
        struct Response {
            voices: Vec<String>,
        }

        let list: Response = res.json().await?;
        Ok(list.voices)
    }

    pub async fn get_rvc_voices_list(&self) -> reqwest::Result<Vec<String>> {
        let res = self
            .client
            .get(self.address.join("api/rvcvoices").unwrap())
            .send()
            .await?;

        #[derive(Deserialize)]
        struct Response {
            rvcvoices: Vec<String>,
        }

        let list: Response = res.json().await?;
        Ok(list.rvcvoices)
    }

    pub async fn get_current_settings(&self) -> reqwest::Result<Settings> {
        let res = self
            .client
            .get(self.address.join("api/currentsettings").unwrap())
            .send()
            .await?;

        res.json().await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub current_model_loaded: String,
    pub manufacturer_name: String,
    pub audio_format: String,
    pub deepspeed_capable: bool,
    pub deepspeed_available: bool,
    pub deepspeed_enabled: bool,
    pub generationspeed_capable: bool,
    pub generationspeed_set: f32,
    pub lowvram_capable: bool,
    pub lowvram_enabled: bool,
    pub pitch_capable: bool,
    pub pitch_set: f32,
    pub repetitionpenalty_capable: bool,
    pub repetitionpenalty_set: f32,
    pub streaming_capable: bool,
    pub temperature_capable: bool,
    pub temperature_set: f32,
    pub ttsengines_installed: bool,
    pub languages_capable: bool,
    pub multivoice_capable: bool,
    pub multimodel_capable: bool,
}
