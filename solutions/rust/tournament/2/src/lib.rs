use std::collections::HashMap;

#[derive(Default)]
pub struct Result {
    wins: u32,
    draws: u32,
    losses: u32,
}

impl Result {
    pub fn matches(&self) -> u32 {
        self.wins + self.draws + self.losses
    }

    pub fn points(&self) -> u32 {
        3 * self.wins + self.draws
    }
}

pub fn tally(match_results: &str) -> String {
    let mut results = HashMap::<String, Result>::new();

    for line in match_results.lines() {
        let mut tokens = line.splitn(3, ";");
        let first = tokens.next().unwrap();
        let second = tokens.next().unwrap();
        let result = tokens.next().unwrap();

        match result {
            "win" => {
                results.entry(first.to_string()).or_default().wins += 1;
                results.entry(second.to_string()).or_default().losses += 1;
            },
            "draw" => {
                results.entry(first.to_string()).or_default().draws += 1;
                results.entry(second.to_string()).or_default().draws += 1;
            },
            "loss" => {
                results.entry(first.to_string()).or_default().losses += 1;
                results.entry(second.to_string()).or_default().wins += 1;
            },
            _ => {}
        }
    }

    let mut points = results.iter().map(
        |(team, result)| {
            (team.as_str(), result.points())
        }
    ).collect::<Vec<(&str, u32)>>();

    points.sort_by(
        |(name_a, a), (name_b, b)| {
            match b.partial_cmp(a).unwrap() {
                std::cmp::Ordering::Equal => name_a.partial_cmp(name_b).unwrap(),
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            }
        }
    );

    let mut result = format!("{:31}|{:^4}| {:^3}| {:^3}| {:^3}|{:>3}", "Team", "MP", "W", "D", "L", "P");

    for (team, _) in points {
        let team_result = results.get(team).unwrap();

        result.push_str("\n");
        result.push_str(
            &format!(
                "{:31}| {:^3}| {:^3}| {:^3}| {:^3}|{:>3}",
                team,
                team_result.matches(),
                team_result.wins,
                team_result.draws,
                team_result.losses,
                team_result.points(),
            )
        )
    }

    result
}