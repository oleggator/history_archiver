use std::time::Duration;

use super::Result;
use crate::model::Visit;
use futures::executor::block_on;
use meilisearch_sdk::client::*;

use super::Destination;

pub struct Meilisearch {
    client: Client,
}

impl Meilisearch {
    const PRIMARY_KEY: &'static str = "visit_time";
    const NAME: &'static str = "meilisearch";

    pub fn new(host: impl Into<String>, api_key: impl Into<String>) -> Meilisearch {
        let client = Client::new(host, api_key);
        Meilisearch { client }
    }
}

impl Destination for Meilisearch {
    fn push_visits(&self, visits: &Vec<Visit>) -> Result<()> {
        let index = self.client.index("visits");

        let task = block_on(async move {
            let task_info = index.add_documents(visits, Some(Self::PRIMARY_KEY)).await?;
            let task = self
                .client
                .wait_for_task(
                    task_info,
                    Some(Duration::from_secs(1)),
                    Some(Duration::from_secs(60)),
                )
                .await?;

            Result::Ok(task)
        })?;

        use meilisearch_sdk::tasks::Task;
        if let Task::Failed { content } = task {
            return Err(Box::new(content.error));
        }

        Ok(())
    }

    fn name(&self) -> &'static str { Self::NAME }
}
