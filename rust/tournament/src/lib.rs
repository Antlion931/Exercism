use std::ops::Neg;
use std::collections::HashMap;
use itertools::Itertools;

enum MatchResult {
    WIN,
    DRAW,
    LOSS,
}

impl Neg for MatchResult {
    type Output = MatchResult;

    fn neg(self) -> Self::Output {
        match self {
            MatchResult::WIN => MatchResult::LOSS,
            MatchResult::DRAW => MatchResult::DRAW,
            MatchResult::LOSS => MatchResult::WIN,
        }
    }
}

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

    pub fn result_of_match(&mut self, r: &MatchResult) {
        match r {
            MatchResult::WIN => self.wins += 1,
            MatchResult::DRAW => self.draws += 1,
            MatchResult::LOSS => self.lost += 1,
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
        let mch_vec = mch.split_terminator(';').collect::<Vec<_>>();
        let team_a = mch_vec[0];
        let team_b = mch_vec[1];

        let match_result = match mch_vec[2] {
            "win" => MatchResult::WIN,
            "loss" => MatchResult::LOSS,
            _ => MatchResult::DRAW,
        };

        results.entry(team_a).or_insert(TeamStatistic::new()).result_of_match(&match_result);
        results.entry(team_b).or_insert(TeamStatistic::new()).result_of_match(&-match_result);
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
