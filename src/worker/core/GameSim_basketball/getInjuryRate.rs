use std::cmp;

pub fn getInjuryRate(baseRate: f64, age: f64, playingThroughInjury: Option<bool>) -> f64 {
    let values = vec![50.0, age];
    let min = values.into_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let mut injuryRate = baseRate * 1.03_f64.powf(min - 26.0);

    if playingThroughInjury.unwrap() {
        injuryRate *= 1.5;
    }

    return injuryRate;
}