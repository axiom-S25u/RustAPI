pub use tungstenite::{connect, WebSocket};
use url::Url;

pub fn connect_to_url(url: &str) -> Result<(WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>, tungstenite::handshake::client::Response), tungstenite::Error> {
    let parsed_url = Url::parse(url).map_err(|e| tungstenite::Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string())))?;
    connect(parsed_url)
}
