use crate::lobby::{Lobby, Participant};

pub fn create_opgg_link(summoners: Vec<Participant>) -> String {
    let mut region = get_common_region(&summoners);
    // Remove any numbers from region
    region.retain(|c| !c.is_numeric());

    let mut opgg_link = format!("https://www.op.gg/multisearch/{}?summoners=", region);
    for summoner in summoners {
        let name_without_spaces = summoner.name.replace(' ', "%20");
        opgg_link.push_str(&name_without_spaces);
        opgg_link.push(',');
    }
    opgg_link.pop();
    opgg_link
}

fn get_common_region(summoners: &Vec<Participant>) -> String {
    // Go through each summoner and find the most common region
    let mut regions = Vec::new();
    for summoner in summoners {
        regions.push(&summoner.region);
    }

    let mut most_common_region = String::new();
    let mut highest_count = 0;
    for region in regions.clone() {
        let count = regions.iter().filter(|&r| r == &region).count();
        if count > highest_count {
            highest_count = count;
            most_common_region = region.clone();
        }
    }

    most_common_region
}

pub fn display_champ_select(lobby: Lobby) {
    if lobby.participants.is_empty() {
        println!("We couldn't find any team mates, try again later.");
        return;
    }

    let mut team_string = String::new();
    for summoner in lobby.participants.iter() {
        team_string.push_str(&summoner.name);
        if summoner.name != lobby.participants.last().unwrap().name {
            team_string.push_str(", ");
        }
    }

    println!("Team: {}", team_string);
    let link = create_opgg_link(lobby.participants);
    match open::that(&link) {
        Ok(_) => {}
        Err(_) => {
            println!("Failed to open link in browser, the link to your lobby is below.");
        }
    }
    println!("{}", link);
}