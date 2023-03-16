use crate::model::Visit;

#[cfg(target_os = "macos")]
pub mod safari;
pub mod firefox;
pub mod chrome;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Source {
    fn get_visits(&self) -> Result<Vec<Visit>>;
    fn name(&self) -> &'static str;
}
