use std::collections::HashSet;

#[allow(non_snake_case)]
fn climbingLeaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    let leaderboard = calculate_overall_rank(ranked);
    println!("{:?}", leaderboard);
    for score in player {
        rank_player(score, &leaderboard);
    }
    Vec::new()
}

fn rank_player(score: &i32, ranking: &Vec<i32>) -> usize {
    let head = ranking[0];
    let mid = ranking.len() % 2;
    let tail = ranking[ranking.len()];
    match score {
        0 => ranking.len(),
        x if x == &head => 1,
        x if x == &tail => ranking.len(),
        x if x > &ranking[mid] => 0,
        _ => 0,
    }
}

fn calculate_overall_rank(rankings: &[i32]) -> Vec<i32> {
    let mut ranking: Vec<i32> = rankings
        .into_iter()
        .fold(HashSet::new(), |mut acc, rank| {
            acc.insert(rank.clone());
            acc
        })
        .into_iter()
        .collect();

    ranking.sort();
    ranking
}
fn main() {
    let ranked = &[100, 100, 50, 40, 40, 20, 10];
    let player = &[50, 50, 75, 100];
    climbingLeaderboard(ranked, player);
}
