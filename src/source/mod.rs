use crate::model::Visit;

pub mod safari;
pub mod firefox;
pub mod chrome;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Source {
    const NAME: &'static str;

    fn get_visits(&self) -> Result<Vec<Visit>>;
}
