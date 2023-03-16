use super::Result;
use super::Source;
use crate::model::Visit;

use ini::Ini;
use rusqlite::types::Type;
use rusqlite::{Connection, OpenFlags};
use std::path::Path;
use std::path::PathBuf;
use time::OffsetDateTime;

const QUERY: &str = "
SELECT
	moz_historyvisits.id AS id,
	moz_historyvisits.visit_date AS visit_time,
	moz_places.title AS title,
	moz_places.url AS url
FROM
	moz_historyvisits
JOIN moz_places ON moz_historyvisits.place_id = moz_places.id
";

pub struct Firefox {
    conn: rusqlite::Connection,
}

impl Firefox {
    const NAME: &'static str = "firefox";

    pub fn new(path: &impl AsRef<Path>) -> Result<Firefox> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        Ok(Firefox { conn })
    }

    pub fn new_default() -> Result<Firefox> {
        let firefox_path = get_default_firefox_dir()?;
        let firefox_profile_path = get_default_profile_path(&firefox_path).unwrap();
        let firefox_db_path = firefox_profile_path.join("places.sqlite");

        Self::new(&firefox_db_path)
    }
}

impl Source for Firefox {
    fn name(&self) -> &'static str { Self::NAME }

    fn get_visits(&self) -> Result<Vec<Visit>> {
        let mut stmt = self.conn.prepare(QUERY)?;
        let visits: Vec<Visit> = stmt
            .query_map((), |row| {
                let visit_time_mcs: i64 = row.get("visit_time")?;
                Ok(Visit {
                    id: row.get("id")?,
                    visit_time: OffsetDateTime::from_unix_timestamp_nanos(
                        visit_time_mcs as i128 * 1_000,
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

pub fn get_default_profile_path(firefox_dir_path: &Path) -> Option<PathBuf> {
    let ini_path = firefox_dir_path.join("profiles.ini");
    let i = Ini::load_from_file(&ini_path).unwrap();

    let profile = i
        .into_iter()
        .find_map(|(section_name, section_body)| match section_name {
            Some(section_name) if section_name == "Profile0" => Some(section_body),
            _ => None,
        })?;

    if profile.get("Default")? != "1" {
        return None;
    }

    let profile_path = Path::new(profile.get("Path")?);

    let is_path_relative = profile.get("IsRelative")? == "1";
    let profile_path = if is_path_relative {
        firefox_dir_path.join(profile_path)
    } else {
        profile_path.to_owned()
    };

    Some(profile_path)
}

#[cfg(target_os = "macos")]
fn get_default_firefox_dir() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")?;
    let firefox_path = Path::new(&home_dir).join("Library/Application Support/Firefox");

    Ok(firefox_path)
}

#[cfg(target_os = "windows")]
fn get_default_firefox_dir() -> Option<PathBuf> {
    let home_dir = std::env::var("APPDATA")?;
    let firefox_path = Path::new(&home_dir).join("Mozilla/Firefox");

    Ok(firefox_path)
}

#[cfg(target_os = "linux")]
fn get_default_firefox_dir() -> Option<PathBuf> {
    let home_dir = std::env::var("HOME")?;
    let firefox_path = Path::new(&home_dir).join(".mozilla/firefox");

    Ok(firefox_path)
}
