use super::Result;
use crate::model::Visit;
// use flate2::Compression;
// use flate2::write::GzEncoder;
// use std::io::prelude::*;

use super::Destination;

pub struct File {
    path: String,
}

impl File {
    pub fn new(path: &str) -> File {
        File {
            path: path.to_owned(),
        }
    }
}

impl Destination for File {
    const NAME: &'static str = "file";

    fn push_visits(&self, visits: &Vec<Visit>) -> Result<()> {
        let file = std::fs::File::create(&self.path)?;
        let writer = file;
        // let mut writer = GzEncoder::new(writer, Compression::default());

        serde_json::to_writer(writer, &visits)?;

        // let json_raw = serde_json::to_vec(&visits).unwrap();
        // writer.write_all(&json_raw).unwrap();
        // writer.finish().unwrap();

        Ok(())
    }
}
