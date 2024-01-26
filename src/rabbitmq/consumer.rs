use amiquip::{ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};

use crate::rabbitmq::config::RabbitmqConfig;

pub fn rabbitmq_consumer() -> Result<()> {
    let rabbit_config = RabbitmqConfig::new().unwrap();

    let rabbit_queue = rabbit_config
        .channel
        .queue_declare(rabbit_config.queue_name, QueueDeclareOptions::default())?;

    // Start a consumer.
    let consumer = rabbit_queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    callback(consumer);

    rabbit_config.connection.close()
}

pub fn callback(consumer: amiquip::Consumer<'_>) {
    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);

                // start callback code
                println!("({:>3}) Received [{}]", i, body);
                // end callback code

                let _ = consumer.ack(delivery);
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
}
