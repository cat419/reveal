use crate::lobby::Participant;

pub fn create_opgg_link(summoners: Vec<Participant>) -> String {
    let mut opgg_link = "https://www.op.gg/multisearch/na?summoners=".to_string();
    for summoner in summoners {
        opgg_link.push_str(&summoner.name);
        opgg_link.push(',');
    }
    opgg_link.pop();
    opgg_link
}