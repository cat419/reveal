mod lobby;
mod utils;

use std::time::Duration;
use colored::Colorize;
use futures_util::StreamExt;
use shaco::error::LcuWebsocketError;
use shaco::model::ws::LcuSubscriptionType::JsonApiEvent;
use shaco::rest::RESTClient;
use shaco::ws::LcuWebsocketClient;
use tokio::time::sleep;
use crate::lobby::Lobby;

const SEPERATOR: &str = "============================";

const ASCII_ART: &str = r#"
                           _
  _ __ _____   _____  __ _| |
 | '__/ _ \ \ / / _ \/ _` | |
 | | |  __/\ V /  __/ (_| | |
 |_|  \___| \_/ \___|\__,_|_|"#;

#[tokio::main]
async fn main() {
    let version = env!("CARGO_PKG_VERSION");
    println!("{} v{}\n{}\nThe source code is available at: https://github.com/steele123/reveal\n", ASCII_ART.cyan(), version, "This will never be charged for, if you paid anything you were scammed.".red());
    println!("{}", "Trying to connect to league client...".yellow());

    loop {
        let client = match RESTClient::new() {
            Ok(client) => client,
            Err(_) => {
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
            println!("{}", "We detected you are in a lobby here is your team!".bright_cyan());
            let link = utils::create_opgg_link(team.participants);
            println!("CTRL + CLICK LINK TO OPEN\n{}", link);
        }

        while let Some(msg) = ws.next().await {
            let client_state = msg.data.to_string().replace('\"', "");
            if client_state == "ChampSelect" {
                println!("{}", "Champ select started, grabbing team mates...".bright_cyan());
                sleep(Duration::from_secs(3)).await;
                let team: Lobby = serde_json::from_value(client.get("/chat/v5/participants/champ-select".to_string()).await.unwrap()).unwrap();
                if team.participants.is_empty() {
                    println!("{}", "We couldn't find any team mates, try again later.".bright_red());
                    continue;
                }

                let mut team_string = String::new();
                for summoner in team.participants.iter() {
                    team_string.push_str(&summoner.name);
                    if summoner.name != team.participants.last().unwrap().name {
                        team_string.push_str(", ");
                    }
                }

                println!("{}\nTeam: {}", SEPERATOR.magenta(), team_string.bright_purple());
                let link = utils::create_opgg_link(team.participants);
                println!("{}\n{}\n{}", "CTRL + CLICK LINK TO OPEN".bright_green(), link.bright_blue(), SEPERATOR.magenta());
                continue;
            }

            println!("Client State Update: {}", client_state.bright_yellow());
        }
    }
}