use rocket::response::status::Created;
use rocket::serde::json::Json;

use bambangshop::Result;
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;

#[post("/subscribe/<product_type>", data = "<subscriber>")]
pub fn subscribe(product_type: String, subscriber: Json<Subscriber>) -> Result<Created<Json<Subscriber>>> {
    match NotificationService::subscribe(product_type.clone(), subscriber.into_inner()) {
        Ok(saved_subscriber) => Ok(Created::new(format!("/subscribe/{}", product_type)).body(Json::from(saved_subscriber))),
        Err(error) => Err(error),
    }
}

#[post("/unsubscribe/<product_type>?<url>")]
pub fn unsubscribe(product_type: String, url: String) -> Result<Json<bool>> {
    NotificationService::unsubscribe(product_type, url).map(Json::from)
}
