use super::Result;
use super::Source;
use crate::model::Visit;

use rusqlite::types::Type;
use rusqlite::{Connection, OpenFlags};
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
    pub fn new(path: &str) -> Result<Firefox> {
        let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        Ok(Firefox { conn })
    }
}

impl Source for Firefox {
    const NAME: &'static str = "firefox";

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
