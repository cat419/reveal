use crate::lobby::Participant;

pub fn create_opgg_link(summoners: Vec<Participant>) -> String {
    let mut region = summoners[0].region.to_lowercase();
    // Remove any numbers from region
    region.retain(|c| !c.is_numeric());

    let mut opgg_link = format!("https://www.op.gg/multisearch/na?summoners={}", region);
    for summoner in summoners {
        let name_without_spaces = summoner.name.replace(' ', "%20");
        opgg_link.push_str(&name_without_spaces);
        opgg_link.push(',');
    }
    opgg_link.pop();
    opgg_link
}