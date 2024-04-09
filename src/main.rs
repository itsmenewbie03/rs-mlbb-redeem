mod redeem;
#[tokio::main]
async fn main() {
    redeem::send_vc("12345", "1234").await;
}
