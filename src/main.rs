use rusbot::app;


#[tokio::main]
async fn main() {
    app::bot::start().await;
    println!("done")
}

