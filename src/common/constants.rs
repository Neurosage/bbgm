pub struct TIEBREAKERS {
    commonOpponentsRecord: String,
    confRecordIfSame: String,
    divRecordIfSame: String,
    divWinner: String,
    headToHeadRecord: String,
    marginOfVictory: String,
    strengthOfVictory: String,
    strengthOfSchedule: String,
    coinFlip: String,
}

impl TIEBREAKERS {
    pub fn new() -> Self {
        return TIEBREAKERS {
            commonOpponentsRecord: "Common Opponents Record".to_string(),
            confRecordIfSame: "Conference Record".to_string(),
            divRecordIfSame: "Division Record".to_string(),
            divWinner: "Division Winner".to_string(),
            headToHeadRecord: "Head-To-Head Record".to_string(),
            marginOfVictory: "Margin of Victory".to_string(),
            strengthOfVictory: "Strength of Victory".to_string(),
            strengthOfSchedule: "Strength of Schedule".to_string(),
            coinFlip: "Coin Flip".to_string(),
        };
    }
}