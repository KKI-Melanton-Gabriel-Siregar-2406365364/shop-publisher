use rocket::http::Status;

use bambangshop::{Result, compose_error_response};
use crate::model::notification::Notification;
use crate::model::subscriber::Subscriber;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(_product_type: String, _subscriber: Subscriber) -> Result<Subscriber> {
        Err(compose_error_response(
            Status::NotImplemented,
            String::from("Subscribe is not implemented yet."),
        ))
    }

    pub fn unsubscribe(_product_type: String, _url: String) -> Result<bool> {
        Err(compose_error_response(
            Status::NotImplemented,
            String::from("Unsubscribe is not implemented yet."),
        ))
    }

    pub fn notify(_product_type: String, _status: String, _product: crate::model::product::Product) -> Result<Vec<Notification>> {
        Err(compose_error_response(
            Status::NotImplemented,
            String::from("Notify is not implemented yet."),
        ))
    }
}
