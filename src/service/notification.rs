use rocket::http::Status;

use bambangshop::{Result, compose_error_response};
use crate::model::notification::Notification;
use crate::model::subscriber::Subscriber;
use crate::repository::subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: String, subscriber: Subscriber) -> Result<Subscriber> {
        if subscriber.url.is_empty() || subscriber.name.is_empty() {
            return Err(compose_error_response(
                Status::BadRequest,
                String::from("Subscriber name and url must not be empty."),
            ));
        }

        let saved_subscriber = SubscriberRepository::add(product_type, subscriber);
        Ok(saved_subscriber)
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
