mod lobby;
mod utils;

use std::time::Duration;
use colored::Colorize;
use futures_util::StreamExt;
use shaco::model::ws::LcuSubscriptionType::JsonApiEvent;
use shaco::rest::RESTClient;
use shaco::ws::LcuWebsocketClient;
use tokio::time::sleep;
use crate::lobby::Lobby;
use crate::utils::display_champ_select;

const ASCII_ART: &str = r#"
                           _
  _ __ _____   _____  __ _| |
 | '__/ _ \ \ / / _ \/ _` | |
 | | |  __/\ V /  __/ (_| | |
 |_|  \___| \_/ \___|\__,_|_|"#;

#[tokio::main]
async fn main() {
    let version = env!("CARGO_PKG_VERSION");
    println!("{} v{}\nThe source code is available at: https://github.com/steele123/reveal\n", ASCII_ART.cyan(), version);
    println!("Made with {} by {}", "❤️".red(), "Steele".bright_yellow());
    println!("{}", "Trying to connect to league client...".yellow());

    let mut connected = false;
    loop {
        let client = match RESTClient::new() {
            Ok(client) => {
                connected = true;
                client
            }
            Err(_) => {
                if connected {
                    println!("{}", "Lost connection to league client, trying to reconnect...".bright_red());
                    connected = false;
                }
                sleep(Duration::from_secs(1)).await;
                continue;
            }
        };

        // The websocket event API will not be opened until a few seconds after the client is opened.
        let mut ws = match LcuWebsocketClient::connect().await {
            Ok(ws) => ws,
            Err(_) => {
                sleep(Duration::from_secs(2)).await;
                LcuWebsocketClient::connect().await.unwrap()
            }
        };

        ws
            .subscribe(JsonApiEvent("/lol-gameflow/v1/gameflow-phase".to_string()))
            .await
            .unwrap();

        println!("{}", "Connected to League Client!".green());
        let team: Lobby = serde_json::from_value(client.get("/chat/v5/participants/champ-select".to_string()).await.unwrap()).unwrap();
        if !team.participants.is_empty() {
            display_champ_select(team);
        }

        while let Some(msg) = ws.next().await {
            let client_state = msg.data.to_string().replace('\"', "");
            if client_state == "ChampSelect" {
                println!("{}", "Champ select started, grabbing team mates...".bright_cyan());
                sleep(Duration::from_secs(3)).await;
                let team: Lobby = serde_json::from_value(client.get("/chat/v5/participants/champ-select".to_string()).await.unwrap()).unwrap();
                display_champ_select(team);
                continue;
            }

            println!("Client State Update: {}", client_state.bright_yellow());
        }
    }
}