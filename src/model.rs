use serde::{Serialize, Serializer};
use time::OffsetDateTime;

pub fn serialize_timestamp<S: Serializer>(
    datetime: &OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    datetime.unix_timestamp_nanos().serialize(serializer)
}

#[derive(Debug, Serialize)]
pub struct Visit {
    ///
    pub id: i64,

    ///
    #[serde(serialize_with = "serialize_timestamp")]
    pub visit_time: OffsetDateTime,

    ///
    pub title: Option<String>,

    ///
    pub url: String,

    ///
    pub domain_expansion: Option<String>,

    ///
    pub status_code: i64,

    ///
    pub tags: Vec<String>,

    ///
    pub source: String,
}
