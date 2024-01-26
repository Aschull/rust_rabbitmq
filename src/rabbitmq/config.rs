use amiquip::{Channel, Connection, Error, Result};
use dotenv::dotenv;
use std::env;

pub struct RabbitmqConfig {
    pub connection: Connection,
    pub channel: Channel,
    pub queue_name: String,
}

impl RabbitmqConfig {
    pub fn new() -> Result<RabbitmqConfig, Error> {
        dotenv().ok();

        let rabbitmq_url = env::var("RABITTMQ_URL").expect("RABITTMQ_URL not set");

        let mut connection = Connection::insecure_open(&rabbitmq_url)?;
        let channel = connection.open_channel(None)?;

        let queue_name = env::var("RABITTMQ_QUEUE_NAME").expect("RABITTMQ_QUEUE_NAME not set");

        Ok(RabbitmqConfig {
            connection,
            channel,
            queue_name,
        })
    }
}
