pub mod cmd;
pub mod stdlib;

#[cfg(feature = "tokio-runtime")]
use tokio;

#[tokio::main]
async fn main()  {
    cmd::init();
}
