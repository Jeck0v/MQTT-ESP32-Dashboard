use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use rumqttc::{AsyncClient, MqttOptions, QoS, Event, Packet};

#[derive(Deserialize)]
struct AuthMessage {
    #[serde(rename = "type")]
    msg_type: String,
    key: String,
}

#[derive(Serialize)]
struct AuthResponse {
    #[serde(rename = "type")]
    msg_type: String,
    status: String,
    message: String,
}

#[derive(Serialize, Clone)]
struct MqttMessage {
    #[serde(rename = "type")]
    msg_type: String,
    topic: String,
    payload: serde_json::Value,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let auth_key = std::env::var("AUTH_KEY")
        .expect("AUTH_KEY must be set in .env or environment variables");

    // MQTT configuration
    let mqtt_host = std::env::var("MQTT_HOST")
        .unwrap_or_else(|_| "localhost".to_string());
    let mqtt_port: u16 = std::env::var("MQTT_PORT")
        .unwrap_or_else(|_| "1883".to_string())
        .parse()
        .expect("MQTT_PORT must be a valid number");
    let mqtt_username = std::env::var("MQTT_USERNAME").ok();
    let mqtt_password = std::env::var("MQTT_PASSWORD").ok();

    println!("Server starting - MQTT broker: {}:{}", mqtt_host, mqtt_port);

    let (mqtt_tx, _mqtt_rx) = broadcast::channel::<MqttMessage>(100);
    let mqtt_tx = Arc::new(mqtt_tx);

    let mut mqttoptions = MqttOptions::new("rust-server", mqtt_host, mqtt_port);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

    // Add MQTT authentication if credentials are provided
    if let (Some(username), Some(password)) = (mqtt_username, mqtt_password) {
        mqttoptions.set_credentials(username, password);
        println!("MQTT authentication configured");
    }

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    let subscribe_client = client.clone();
    tokio::spawn(async move {
        println!("Subscribing to MQTT topics...");
        if let Err(e) = subscribe_client.subscribe("classroom/+/telemetry", QoS::AtLeastOnce).await {
            println!("Failed to subscribe: {}", e);
        } else {
            println!("Successfully subscribed to classroom/+/telemetry");
        }
    });

    let mqtt_tx_clone = mqtt_tx.clone();
    tokio::spawn(async move {
        println!("Starting MQTT event loop...");
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::Publish(publish))) => {
                    let topic = publish.topic.clone();
                    let payload = String::from_utf8_lossy(&publish.payload).to_string();

                    println!("MQTT message received on topic {}: {}", topic, payload);

                    if let Ok(json_payload) = serde_json::from_str::<serde_json::Value>(&payload) {
                        let mqtt_msg = MqttMessage {
                            msg_type: "mqtt".to_string(),
                            topic: topic.clone(),
                            payload: json_payload,
                        };

                        let _ = mqtt_tx_clone.send(mqtt_msg);
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    println!("MQTT Error: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    });
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("WebSocket server listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let mqtt_rx = mqtt_tx.subscribe();
        let auth_key_clone = auth_key.clone();
        tokio::spawn(handle_connection(stream, mqtt_rx, auth_key_clone));
    }
}

async fn handle_connection(stream: TcpStream, mut mqtt_rx: broadcast::Receiver<MqttMessage>, auth_key: String) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    println!("New WebSocket connection");

    let (mut write, mut read) = ws_stream.split();

    let authenticated = if let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Auth attempt received: {}", text);

                match serde_json::from_str::<AuthMessage>(&text) {
                    Ok(auth_msg) if auth_msg.msg_type == "auth" => {
                        if auth_msg.key == auth_key {
                            println!("Authentication successful");
                            let response = AuthResponse {
                                msg_type: "auth_response".to_string(),
                                status: "success".to_string(),
                                message: "Authenticated successfully".to_string(),
                            };
                            if let Ok(response_json) = serde_json::to_string(&response) {
                                let _ = write.send(Message::Text(response_json)).await;
                            }
                            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                            let welcome_msg = "Hello from the other side";
                            let _ = write.send(Message::Text(welcome_msg.to_string())).await;
                            println!("Welcome message sent to client");

                            true
                        } else {
                            println!("Authentication failed: invalid key");

                            let response = AuthResponse {
                                msg_type: "auth_response".to_string(),
                                status: "error".to_string(),
                                message: "Invalid authentication key".to_string(),
                            };

                            if let Ok(response_json) = serde_json::to_string(&response) {
                                let _ = write.send(Message::Text(response_json)).await;
                            }
                            let _ = write.send(Message::Close(None)).await;
                            false
                        }
                    }
                    _ => {
                        println!("Authentication failed: invalid message format");

                        let response = AuthResponse {
                            msg_type: "auth_response".to_string(),
                            status: "error".to_string(),
                            message: "Authentication required".to_string(),
                        };

                        if let Ok(response_json) = serde_json::to_string(&response) {
                            let _ = write.send(Message::Text(response_json)).await;
                        }
                        let _ = write.send(Message::Close(None)).await;
                        false
                    }
                }
            }
            _ => {
                println!("Connection closed before authentication");
                false
            }
        }
    } else {
        println!("No authentication message received");
        false
    };

    if !authenticated {
        return;
    }
    let write = Arc::new(Mutex::new(write));
    let write_clone = write.clone();

    let mqtt_task = tokio::spawn(async move {
        while let Ok(mqtt_msg) = mqtt_rx.recv().await {
            println!("Forwarding MQTT message to WebSocket client");
            if let Ok(json) = serde_json::to_string(&mqtt_msg) {
                let mut write = write_clone.lock().await;
                if let Err(e) = write.send(Message::Text(json)).await {
                    println!("Error sending MQTT message to client: {}", e);
                    break;
                }
            }
        }
    });

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Received: {}", text);

                let mut write = write.lock().await;
                if let Err(e) = write.send(Message::Text("Healthy".to_string())).await {
                    println!("Error sending message: {}", e);
                    break;
                }
            }
            Ok(Message::Close(_)) => {
                println!("Connection closed");
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    mqtt_task.abort();
}
