use interface::InterfaceBuilder;

mod alert;
mod controller;
mod interface;
mod pomodoro;

#[tokio::main]
async fn main() {
    InterfaceBuilder::build().await.spawn_and_run().await;
}
