use serde;
use serde::Deserialize;

use crate::destination::file::File;
use crate::destination::meilisearch::Meilisearch;
use crate::destination::Destination;
use crate::source::chrome::Chrome;
use crate::source::firefox::Firefox;
use crate::source::safari::Safari;
use crate::source::Source;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub sources: Vec<SourceConfig>,
    pub destinations: Vec<DestinationConfig>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SourceConfig {
    #[serde(rename = "safari")]
    Safari(SafariConfig),

    #[serde(rename = "firefox")]
    Firefox(FirefoxConfig),

    #[serde(rename = "chrome")]
    Chrome(ChromeConfig),
}

impl SourceConfig {
    pub fn create(&self) -> Result<Box<dyn Source>> {
        let src: Box<dyn Source> = match self {
            SourceConfig::Safari(_src_config) => Box::new(Safari::new_default()?),
            SourceConfig::Firefox(_src_config) => Box::new(Firefox::new_default()?),
            SourceConfig::Chrome(_src_config) => Box::new(Chrome::new_default()?),
        };
        Ok(src)
    }
}

#[derive(Deserialize, Debug)]
pub struct SafariConfig {
    pub path: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct FirefoxConfig {
    pub path: Option<String>,
    pub profile: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ChromeConfig {
    pub path: Option<String>,
    pub profile: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DestinationConfig {
    #[serde(rename = "meilisearch")]
    Meilisearch(MeilisearchConfig),

    #[serde(rename = "file")]
    File(FileConfig),
}

impl DestinationConfig {
    pub fn create(&self) -> Result<Box<dyn Destination>> {
        let dst: Box<dyn Destination> = match self {
            DestinationConfig::Meilisearch(destination_config) => Box::new(Meilisearch::new(
                &destination_config.address,
                &destination_config.api_key,
            )),
            DestinationConfig::File(file_config) => Box::new(File::new(&file_config.path)),
        };
        Ok(dst)
    }
}

#[derive(Deserialize, Debug)]
pub struct MeilisearchConfig {
    pub address: String,
    pub api_key: String,
}

#[derive(Deserialize, Debug)]
pub struct FileConfig {
    pub path: String,
    pub encoding: FileEncodingConfig,
    pub compression: FileCompressionConfig,
}

#[derive(Deserialize, Debug)]
pub enum FileEncodingConfig {
    #[serde(rename = "json")]
    JSON,
}

#[derive(Deserialize, Debug)]
pub enum FileCompressionConfig {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "json")]
    GZip,
}
