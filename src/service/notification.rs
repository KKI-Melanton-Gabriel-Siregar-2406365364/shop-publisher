use rocket::http::Status;
use std::thread;

use bambangshop::{Result, compose_error_response};
use crate::model::notification::Notification;
use crate::model::product::Product;
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

    pub fn unsubscribe(product_type: String, url: String) -> Result<bool> {
        if url.is_empty() {
            return Err(compose_error_response(
                Status::BadRequest,
                String::from("Query parameter `url` must not be empty."),
            ));
        }

        Ok(SubscriberRepository::delete(product_type, url))
    }

    pub fn notify(product_type: String, status: String, product: Product) -> Result<Vec<Notification>> {
        let normalized_product_type = product_type.to_uppercase();
        let subscribers = SubscriberRepository::list_all(normalized_product_type.clone());
        let mut notifications: Vec<Notification> = vec![];
        let mut workers = vec![];

        for subscriber in subscribers {
            let notification = Notification {
                product_title: product.title.clone(),
                product_url: product.get_url(),
                product_type: normalized_product_type.clone(),
                subscriber_name: subscriber.name.clone(),
                status: status.clone(),
            };

            notifications.push(notification.clone());
            workers.push(thread::spawn(move || {
                let _ = subscriber.update(notification);
            }));
        }

        for worker in workers {
            if worker.join().is_err() {
                return Err(compose_error_response(
                    Status::InternalServerError,
                    String::from("Failed to notify one or more subscribers."),
                ));
            }
        }

        Ok(notifications)
    }
}
