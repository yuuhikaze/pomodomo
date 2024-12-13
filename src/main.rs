use interface::InterfaceBuilder;

mod interface;
mod controller;

#[tokio::main]
async fn main() {
    InterfaceBuilder::build().await.spawn_and_run().await;
}
