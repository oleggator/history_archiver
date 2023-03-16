use super::Result;
use super::Source;
use crate::model::Visit;
use rusqlite::types::Type;
use rusqlite::{Connection, OpenFlags};
use std::path::Path;
use std::path::PathBuf;
use time::OffsetDateTime;

const QUERY: &str = "
SELECT
	visits.id AS id,
	visits.visit_time AS visit_time,
	urls.title AS title,
	urls.url AS url
FROM
	visits
JOIN urls ON visits.url = urls.id
";

const TIME_BASE_MCS: i64 = 11_644_473_600_000_000;

pub struct Chrome {
    conn: rusqlite::Connection,
}

impl Chrome {
    const NAME: &'static str = "chrome";
    
    pub fn new(path: &impl AsRef<Path>) -> Result<Chrome> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        Ok(Chrome { conn })
    }

    pub fn new_default() -> Result<Chrome> {
        let chrome_dir = get_default_chrome_dir()?;
        let db_path = chrome_dir.join("Default/History");

        Self::new(&db_path)
    }
}

impl Source for Chrome {
    fn name(&self) -> &'static str { Self::NAME }

    fn get_visits(&self) -> Result<Vec<Visit>> {
        let mut stmt = self.conn.prepare(QUERY)?;
        let visits: Vec<Visit> = stmt
            .query_map((), |row| {
                let visit_time_mcs: i64 = row.get("visit_time")?;
                Ok(Visit {
                    id: row.get("id")?,
                    visit_time: OffsetDateTime::from_unix_timestamp_nanos(
                        (visit_time_mcs - TIME_BASE_MCS) as i128 * 1_000,
                    )
                    .map_err(|err| {
                        rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(err))
                    })?,
                    title: row.get("title")?,
                    url: row.get("url")?,
                    tags: vec![],
                    source: Self::NAME.to_owned(),
                })
            })?
            .map(|visit| visit.unwrap())
            .collect();

        Ok(visits)
    }
}

#[cfg(target_os = "macos")]
fn get_default_chrome_dir() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")?;
    let chrome_dir = Path::new(&home_dir).join("Library/Application Support/Google/Chrome");

    Ok(chrome_dir)
}

#[cfg(target_os = "windows")]
fn get_default_chrome_dir() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")?;
    let chrome_dir = Path::new(&home_dir).join("/AppData/Local/Google/Chrome/User Data");

    Ok(chrome_dir)
}

#[cfg(target_os = "linux")]
fn get_default_chrome_dir() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")?;
    let chrome_dir = Path::new(&home_dir).join(".config/google-chrome");

    Ok(chrome_dir)
}
