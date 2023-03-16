use std::path::{Path, PathBuf};

use super::Result;
use crate::model::Visit;
// use flate2::Compression;
// use flate2::write::GzEncoder;
// use std::io::prelude::*;

use super::Destination;

pub struct File {
    path: PathBuf,
}

impl File {
    const NAME: &'static str = "file";

    pub fn new(path: &impl AsRef<Path>) -> File {
        File {
            path: path.as_ref().to_owned(),
        }
    }
}

impl Destination for File {
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

    fn name(&self) -> &'static str { Self::NAME }
}
