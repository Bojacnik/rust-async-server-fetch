mod server_crasher;

use server_crasher::start;

#[tokio::main]
async fn main() {
    start(2).await;
}

