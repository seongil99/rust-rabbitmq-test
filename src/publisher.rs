use lapin::{
    message::{BasicReturnMessage, Delivery},
    options::*,
    protocol::{AMQPErrorKind, AMQPSoftError},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use serde_json::json;
use tracing::info;

pub fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

    async_global_executor::block_on(async {
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .expect("connection error");

        info!("CONNECTED");

        //send channel
        let channel_a = conn.create_channel().await.expect("create_channel");
        info!(state=?conn.status().state());

        //create the hello queue
        let queue = channel_a
            .queue_declare(
                "to_rust",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("queue_declare");
        info!(state=?conn.status().state());
        info!(?queue, "Declared queue");

        channel_a
            .confirm_select(ConfirmSelectOptions::default())
            .await
            .expect("confirm_select");
        info!(state=?conn.status().state());
        info!("Enabled publisher-confirms");

        info!("will publish");
        let payload = b"Hello world!";
        let data = json!({
            "lang": "rust",
            "code": "println!(\"Hello world!\");",
        });
        let v = serde_json::to_vec(&data).unwrap();

        let confirm = channel_a
            .basic_publish(
                "",
                "to_rust",
                BasicPublishOptions::default(),
                &v,
                BasicProperties::default(),
            )
            .await
            .expect("basic_publish")
            .await // Wait for this specific ack/nack
            .expect("publisher-confirms");
        assert!(confirm.is_ack());
        assert_eq!(confirm.take_message(), None);
        info!(state=?conn.status().state());

        // ... and wait for all pending ack/nack afterwards instead of individually in the above loop
        let returned = channel_a
            .wait_for_confirms()
            .await
            .expect("wait for confirms");
        assert!(returned.is_empty());

        let confirm = channel_a
            .basic_publish(
                "",
                "unroutable-routing-key-for-tests",
                BasicPublishOptions {
                    mandatory: true,
                    ..BasicPublishOptions::default()
                },
                payload,
                BasicProperties::default().with_priority(42),
            )
            .await
            .expect("basic_publish")
            .await // Wait for this specific ack/nack
            .expect("publisher-confirms");
        assert!(confirm.is_ack());
        let message = confirm.take_message().unwrap();
        assert_eq!(
            message,
            BasicReturnMessage {
                delivery: Delivery {
                    delivery_tag: 0,
                    exchange: "".into(),
                    routing_key: "unroutable-routing-key-for-tests".into(),
                    redelivered: false,
                    properties: BasicProperties::default().with_priority(42),
                    data: payload.to_vec(),
                    acker: Default::default(),
                },
                reply_code: 312,
                reply_text: "NO_ROUTE".into(),
            }
        );
        let error = message.error().unwrap();
        assert_eq!(error.kind(), &AMQPErrorKind::Soft(AMQPSoftError::NOROUTE));

        let _ = channel_a;
    });
}
