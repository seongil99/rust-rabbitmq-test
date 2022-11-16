mod callback_function;
mod consumer;
mod publisher;

fn main() {
    tracing_subscriber::fmt::init();
    publisher::main();
    consumer::main();
}
