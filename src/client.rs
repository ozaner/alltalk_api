use std::io::Read;

use reqwest::Url;
use serde::Deserialize;
use stream_download::{storage::memory::MemoryStorageProvider, StreamDownload};

use crate::StreamingWav;

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

    pub async fn generate_tts_stream(
        &self,
        text: impl AsRef<str>,
        voice: impl AsRef<str>,
        language: impl AsRef<str>,
    ) -> Result<StreamingWav<impl Read>, Box<dyn std::error::Error>> {
        let mut url = self.address.join("api/tts-generate-streaming").unwrap();
        url.query_pairs_mut()
            .append_pair("text", text.as_ref())
            .append_pair("voice", voice.as_ref())
            .append_pair("language", language.as_ref())
            .append_pair("output_file", "stream_output.wav") //no need to change this...
            .finish();

        let reader = StreamDownload::new_http(
            url,
            MemoryStorageProvider,
            stream_download::Settings::default(),
        )
        .await?;

        Ok(StreamingWav::new(reader)?)
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

    pub async fn stop_generation(&self) -> reqwest::Result<bool> {
        let res = self
            .client
            .put(self.address.join("api/stop-generation").unwrap())
            .send()
            .await?;

        #[derive(Deserialize)]
        struct Response {
            message: String,
        }

        let res: Response = res.json().await?;
        Ok(res.message == "Cancelling current TTS generation")
    }

    pub async fn reload_config(&self) -> reqwest::Result<bool> {
        let res = self
            .client
            .put(self.address.join("api/reload-config").unwrap())
            .send()
            .await?;

        Ok(res.text().await? == "Config file reloaded successfully")
    }

    pub async fn set_model(&self, model: impl AsRef<str>) -> reqwest::Result<bool> {
        let res = self
            .client
            .put(self.address.join("api/reload").unwrap())
            .query(&[("tts_method", model.as_ref())])
            .send()
            .await?;

        #[derive(Deserialize)]
        struct Response {
            status: String,
        }

        let res: Response = res.json().await?;
        Ok(res.status == "model-success")
    }

    pub async fn set_deepspeed(&self, enable: bool) -> reqwest::Result<SetValueResponse> {
        let res = self
            .client
            .post(self.address.join("api/deepspeed").unwrap())
            .query(&[("new_deepspeed_value", if enable { "True" } else { "False" })])
            .send()
            .await?;

        #[derive(Deserialize)]
        struct Response {
            status: String,
            message: Option<String>,
        }

        let res: Response = res.json().await?;
        let success = res.status.ends_with("success");
        if success {
            Ok(SetValueResponse::Success(if let Some(msg) = res.message {
                if enable {
                    msg == "DeepSpeed is already enabled."
                } else {
                    msg == "DeepSpeed is already disabled."
                }
            } else {
                false
            }))
        } else {
            Ok(SetValueResponse::Error)
        }
    }

    pub async fn set_low_vram(&self, enable: bool) -> reqwest::Result<SetValueResponse> {
        let res = self
            .client
            .post(self.address.join("api/lowvramsetting").unwrap())
            .query(&[("new_low_vram_value", if enable { "True" } else { "False" })])
            .send()
            .await?;

        #[derive(Deserialize)]
        struct Response {
            status: String,
            message: Option<String>,
        }

        let res: Response = res.json().await?;
        let success = res.status.ends_with("success");
        if success {
            Ok(SetValueResponse::Success(if let Some(msg) = res.message {
                if enable {
                    msg == "[AllTalk Model] LowVRAM is already enabled."
                } else {
                    msg == "[AllTalk Model] LowVRAM is already disabled."
                }
            } else {
                false
            }))
        } else {
            Ok(SetValueResponse::Error)
        }
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

#[derive(Debug)]
pub enum SetValueResponse {
    Success(bool),
    Error,
}
