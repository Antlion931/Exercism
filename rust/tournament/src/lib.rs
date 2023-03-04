use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
struct TeamStatistic {
    pub wins: i8,
    pub draws: i8,
    pub lost: i8,
}

impl TeamStatistic {
    pub fn new() -> Self {
        Self { wins: 0, draws: 0, lost: 0 }
    }

    pub fn result_of_oponent_match(&mut self, r: &str) {
        match r {
            "loss" => self.wins += 1,
            "draw" => self.draws += 1,
            "win" => self.lost += 1,
            _ => {},
        }
    }

    pub fn result_of_match(&mut self, r: &str) {
        match r {
            "win" => self.wins += 1,
            "draw" => self.draws += 1,
            "loss" => self.lost += 1,
            _ => {},
        }
    }

    pub fn count(&self) -> i8 {
        self.wins + self.draws + self.lost
    }

    pub fn points(&self) -> i8 {
        self.wins * 3 + self.draws
    }   
}
    
pub fn tally(match_results: &str) -> String {
    let mut results = HashMap::new();

    for mch in match_results.lines() {
        if let [team_a, team_b, result, ..] =  mch.split(";").collect::<Vec<_>>().as_slice() {
            results.entry(team_a.clone()).or_insert(TeamStatistic::new()).result_of_match(result);
            results.entry(team_b.clone()).or_insert(TeamStatistic::new()).result_of_oponent_match(result);
        }
    }
    
    let mut result = Vec::new();
    
    for r in results.keys().sorted()  {
        result.push(format!("{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", r, results[r].count(), results[r].wins, results[r].draws, results[r].lost, results[r].points()));
    }
    result.reverse();
    result.sort_by(|a, b| a.split_at(53).1.partial_cmp(b.split_at(53).1).unwrap());
    result.reverse();
    result.insert(0, format!("{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", "Team", "MP", "W", "D", "L", "P"));
    result.join("\n")
}
