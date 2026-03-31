use rocket::serde::json::Json;

use bambangshop::Result;
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;

#[post("/subscribe/<product_type>", data = "<subscriber>")]
pub fn subscribe(product_type: String, subscriber: Json<Subscriber>) -> Result<Json<Subscriber>> {
    NotificationService::subscribe(product_type, subscriber.into_inner()).map(Json::from)
}

#[post("/unsubscribe/<product_type>?<url>")]
pub fn unsubscribe(product_type: String, url: String) -> Result<Json<bool>> {
    NotificationService::unsubscribe(product_type, url).map(Json::from)
}
