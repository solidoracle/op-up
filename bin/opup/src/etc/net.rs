use std::net::{SocketAddr, TcpStream};
use std::thread;
use eyre::Result;

/// Wait for a port to come up.
pub fn wait_up(port: u16, retries: u32, wait_secs: u64) -> Result<()> {
    for _ in 0..retries {
        tracing::debug!(target: "opup", "Trying 127.0.0.1:{}", port);
        if let Ok(stream) = TcpStream::connect(SocketAddr::from(([127, 0, 0, 1], port))) {
            drop(stream);
            tracing::debug!(target: "opup", "Connected 127.0.0.1:{}", port);
            return Ok(());
        }
        thread::sleep(Duration::from_secs(wait_secs));
    }

    eyre::bail!("Timed out waiting for port {}.", port);

    Err(eyre::eyre!("Timed out waiting for port {}.", port))
}

