use dashmap::DashMap;
use lazy_static::lazy_static;

use crate::model::subscriber::Subscriber;

// Singleton of Database
lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, Vec<Subscriber>> = DashMap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
    pub fn add(product_type: String, subscriber: Subscriber) -> Subscriber {
        let normalized_product_type = product_type.to_uppercase();
        SUBSCRIBERS
            .entry(normalized_product_type)
            .and_modify(|subscribers| subscribers.push(subscriber.clone()))
            .or_insert_with(|| vec![subscriber.clone()]);

        subscriber
    }

    pub fn list_all(product_type: String) -> Vec<Subscriber> {
        let _ = product_type;
        vec![]
    }

    pub fn delete(product_type: String, url: String) -> bool {
        let _ = (product_type, url);
        false
    }
}
