pub fn update_elo(p1: i32, p2: i32, p1_won: bool) -> (i32, i32) {
    let k = 32.0;

    let expected_1 = 1.0 / (1.0 + 10f64.powf((p2 - p1) as f64 / 400.0));
    let score_1 = if p1_won { 1.0 } else { 0.0 };

    let new_p1 = p1 as f64 + k * (score_1 - expected_1);
    let new_p2 = p2 as f64 + k * ((1.0 - score_1) - (1.0 - expected_1));

    (new_p1.round() as i32, new_p2.round() as i32)
}
