

pub struct PlayerInjury {
    gamesRemaining: u8,
    injuryType: String,
    score: Option<u8>
}

pub struct GameAttributesLeague {
    pub foulsNeededToFoulOut: i32,
    pub numPlayersOnCourt: i32,
    pub quarterLength: f64,
    pub disableInjuries: bool,
    pub pace: f64,
    pub foulsUntilBonus: Vec<i32>,
    pub foulRateFactor: f64,
    pub turnoverFactor: f64,
}

pub struct GameAttributesNonLeague {
    lid: Option<u8>
}

pub struct GameAttributesWithHistory<T> {
    start: i32,
    value: T
}

pub struct Conf {
    cid: u8,
    name: String,
}

pub struct Div {
    did: u8,
    cid: u8,
    name: String,
}

pub struct GameAttributesLeagueWithHistory {
    pub game_attributes_league: GameAttributesLeague,
    confs: GameAttributesWithHistory<Vec<Conf>>,
    divs: GameAttributesWithHistory<Vec<Div>>,
    numGamesPlayoffSeries: Vec<u8>,
    numPlayoffByes: u8,
    otl: bool,
    playoffsNumTeamsDiv: u8,
    pointsFormula: String,
    tiebreakers: String,
    ties: bool,
}

pub struct GameAttributes {
    pub nonLeague: Option<GameAttributesNonLeague>,
    pub leagueWithHistory: Option<GameAttributesLeagueWithHistory>
}

impl GameAttributes {
    pub fn new(noHistory:Option<bool>) -> Self {
        let mut game_attributes:GameAttributes = GameAttributes {
            nonLeague: None,
            leagueWithHistory: None
        }; 
        if (noHistory.is_some() && noHistory.unwrap()) {
            game_attributes.nonLeague = Some(GameAttributesNonLeague {
                lid: None
            });
            game_attributes.leagueWithHistory = None;
            return game_attributes;
        } else {
            game_attributes.leagueWithHistory = Some(GameAttributesLeagueWithHistory {
                game_attributes_league: GameAttributesLeague {
                    foulsNeededToFoulOut: 6,
                    numPlayersOnCourt: 5
                },
                confs: GameAttributesWithHistory {
                    start: 0,
                    value: Vec::new()
                },
                divs: GameAttributesWithHistory {
                    start: 0,
                    value: Vec::new()
                },
                numGamesPlayoffSeries: Vec::new(),
                numPlayoffByes: 0,
                otl: false,
                playoffsNumTeamsDiv: 0,
                pointsFormula: "".to_string(),
                tiebreakers: "".to_string(),
                ties: false
            });
        }
        return game_attributes;
    }
}