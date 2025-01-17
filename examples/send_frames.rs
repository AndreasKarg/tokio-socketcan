use futures_timer::Delay;
use std::time::Duration;
use tokio;
use tokio_socketcan::{CanFrame, CanSocket, Error};
use socketcan::{EmbeddedFrame, StandardId};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let socket_tx = CanSocket::open("vcan0").unwrap();

    loop {
        let id = StandardId::new(0x1).unwrap();
        let frame = CanFrame::new(id, &[0]).unwrap();
        println!("Writing on vcan0");
        socket_tx.write_frame(frame)?.await?;
        println!("Waiting 3 seconds");
        Delay::new(Duration::from_secs(3)).await;
    }
}
