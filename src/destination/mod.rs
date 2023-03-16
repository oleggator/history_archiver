use crate::model::Visit;

pub mod file;
pub mod meilisearch;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Destination {
    fn push_visits(&self, visits: &Vec<Visit>) -> Result<()>;
    fn name(&self) -> &'static str;
}
