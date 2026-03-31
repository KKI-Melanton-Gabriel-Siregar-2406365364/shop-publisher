use rocket::serde::{Deserialize, Serialize};

use bambangshop::REQWEST_CLIENT;
use crate::model::notification::Notification;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub name: String,
    pub url: String,
}

impl Subscriber {
    pub fn update(&self, payload: Notification) -> bool {
        let request_url = self.url.clone();
        let runtime = rocket::tokio::runtime::Runtime::new();
        if runtime.is_err() {
            return false;
        }

        runtime.unwrap().block_on(async move {
            REQWEST_CLIENT
                .post(request_url)
                .json(&payload)
                .send()
                .await
                .map(|response| response.status().is_success())
                .unwrap_or(false)
        })
    }
}
