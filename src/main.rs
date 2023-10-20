use futures::stream::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;
use std::error::Error;
use std::net::ToSocketAddrs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "wss://stream.binance.com:9443/ws/btcusdt@trade"; // Замените на URL вашего WebSocket-сервера
    let url = Url::parse(url)?;

    let host = url.host_str().expect("Failed to get host");
    let port = url.port_or_known_default().unwrap();

    let addr = format!("{}:{}", host, port);
    let socket_addr = addr.to_socket_addrs()?.next().unwrap();

    let ws_stream = TcpStream::connect(&socket_addr).await?;

    let ws_stream = tokio::time::timeout(std::time::Duration::from_secs(10), accept_async(ws_stream))
    .await
    .expect("Failed to perform WebSocket handshake")
    .expect("WebSocket handshake timed out");
   
    println!("rudsfgsgsd");
    let (_write, mut read) = ws_stream.split();

    // Здесь вы можете отправить сообщение на сервер, если это необходимо.
    // Например:
    // write.send(Message::text("Привет, сервер!")).await?;

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                match msg {
                    Message::Text(text) => {
                        println!("Получено текстовое сообщение: {}", text);
                        // Обработка полученного текстового сообщения
                    }
                    Message::Binary(data) => {
                        println!("Получены бинарные данные: {:?}", data);
                        // Обработка полученных бинарных данных
                    }
                    _ => {
                        // Обработка других типов сообщений, если необходимо
                    }
                }
            }
            Err(e) => {
                eprintln!("Ошибка при получении сообщения: {}", e);
                // Обработка ошибки
            }
        }
    }

    Ok(())
}
// "wss://stream.binance.com:9443/ws/btcusdt@trade"