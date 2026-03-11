mod ring;

use ring::myrotio;
// fn main() {
//     myrotio::sub_main();
// }

#[tokio::main]
async fn main() {
    myrotio::sub_main().await;
}
