use std::path::Path;

use super::Result;
use super::Source;
use crate::model::Visit;
use rusqlite::types::Type;

use rusqlite::{Connection, OpenFlags};
use time::OffsetDateTime;

const QUERY: &str = "
SELECT
	history_visits.id as id,
	history_visits.visit_time as visit_time,
	history_visits.title as title,
	history_items.url as url,
	(
		SELECT json_group_array(history_tags.title) from history_items_to_tags
		JOIN history_tags ON history_items_to_tags.tag_id = history_tags.id
		WHERE history_items_to_tags.history_item = history_items.id
	) as tags
FROM
	history_visits
	JOIN history_items ON history_visits.history_item = history_items.id
";

const NSDATE_OFFSET_S: i128 = 978_307_200;

pub struct Safari {
    conn: rusqlite::Connection,
}

impl Safari {
    const NAME: &'static str = "safari";

    pub fn new(path: &impl AsRef<Path>) -> Result<Safari> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        Ok(Safari { conn })
    }

    pub fn new_default() -> Result<Safari> {
        let home_dir = std::env::var("HOME")?;
        let db_path = Path::new(&home_dir).join("Library/Safari/History.db");

        Self::new(&db_path)
    }
}

impl Source for Safari {
    fn name(&self) -> &'static str { Self::NAME }

    fn get_visits(&self) -> Result<Vec<Visit>> {
        let mut stmt = self.conn.prepare(QUERY)?;
        let visits: Vec<Visit> = stmt
            .query_map((), |row| {
                let tags_json: String = row.get("tags")?;

                let visit_time_nsdate: f64 = row.get("visit_time")?;
                let visit_time_ts_ns: i128 =
                    (visit_time_nsdate * 1_000_000_000.) as i128 + NSDATE_OFFSET_S * 1_000_000_000;

                Ok(Visit {
                    id: row.get("id")?,
                    visit_time: OffsetDateTime::from_unix_timestamp_nanos(visit_time_ts_ns)
                        .map_err(|err| {
                            rusqlite::Error::FromSqlConversionFailure(0, Type::Real, Box::new(err))
                        })?,
                    title: row.get("title")?,
                    url: row.get("url")?,
                    tags: serde_json::from_str(&tags_json).map_err(|err| {
                        rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(err))
                    })?,
                    source: Self::NAME.to_owned(),
                })
            })?
            .map(|visit| visit.unwrap())
            .collect();

        Ok(visits)
    }
}
