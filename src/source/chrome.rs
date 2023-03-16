use super::Result;
use super::Source;
use crate::model::Visit;

use rusqlite::types::Type;
use rusqlite::{Connection, OpenFlags};
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
    pub fn new(path: &str) -> Result<Chrome> {
        let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        Ok(Chrome { conn })
    }
}

impl Source for Chrome {
    const NAME: &'static str = "chrome";

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
