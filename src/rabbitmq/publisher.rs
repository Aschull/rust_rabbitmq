use amiquip::{Exchange, Publish, Result};

use crate::rabbitmq::config::RabbitmqConfig;

pub fn rabbitmq_publisher(message: String, routing_key: String) -> Result<()> {
    let rabbit_config = RabbitmqConfig::new().unwrap();

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&rabbit_config.channel);

    // Publish a message to the "hello" queue.
    exchange.publish(Publish::new(message.as_bytes(), routing_key))?;

    rabbit_config.connection.close()
}
