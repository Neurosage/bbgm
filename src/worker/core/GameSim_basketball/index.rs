use crate::common::types::GameAttributesLeagueWithHistory;
use crate::common::types::PlayerInjury;
use crate::worker::util::g::G;
use crate::worker::util::helpers;
use rand::prelude::*;
use core::num;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;
use std::slice::SliceIndex;
use crate::common::types::GameAttributes;
use crate::worker::core::GameSim_basketball::getInjuryRate::getInjuryRate;

const rng: ThreadRng = rand::thread_rng();
enum PlayType {
    Ast,
    BlkAtRim,
    BlkLowPost,
    BlkMidRange,
    BlkTp,
    Drb,
    FgaAtRim,
    FgaLowPost,
    FgaMidRange,
    FgaTp,
    FgAtRim,
    FgAtRimAndOne,
    FgLowPost,
    FgLowPostAndOne,
    FgMidRange,
    FgMidRangeAndOne,
    FoulOut,
    Ft,
    GameOver,
    Injury,
    JumpBall,
    MissAtRim,
    MissFt,
    MissLowPost,
    MissMidRange,
    MissTp,
    Orb,
    Overtime,
    PfNonShooting,
    PfBonus,
    PfFG,
    PfTP,
    PfAndOne,
    Quarter,
    Stl,
    Sub,
    Tov,
    Tp,
    TpAndOne
}

struct Play {
    play_type: PlayType,
    team: TeamNum,
    on: i32,
    off: i32
}

impl Play {
    fn new(play_type: PlayType, team: TeamNum, on: i32, off: i32) -> Self {
        Play {
            play_type,
            team,
            on,
            off
        }
    }
}

struct Synergy {
    def: f64,
    off: f64,
    reb: f64
}

enum ShotType {
    AtRim,
    Ft,
    LowPost,
    MidRange,
    ThreePointer
}

struct Stat {
    ast: u16,
    ba: u16,
    benchTime: f64,
    blk: u16,
    courtTime: f64,
    drb: u16,
    energy: f64,
    fg: u16,
    fgAtRim: u16,
    fgLowPost: u16,
    fgMidRange: u16,
    fga: u16,
    fgaAtRim: u16,
    fgaLowPost: u16,
    fgaMidRange: u16,
    ft: u16,
    fta: u16,
    gs: bool,
    min: f64,
    orb: u16,
    pf: u16,
    pts: u16,
    stl: u16,
    tov: u16,
    tp: u16,
    tpa: u16
}

type PlayerNumOnCourt = i32;

type TeamNum = i32;

struct TeamCompositeRating {
    ratings: HashMap<String, f64>
}

impl TeamCompositeRating {
    fn insert(&self, key: String, value: f64) {
        self.ratings.insert(key, value);
    }

    fn add(&self, key: String, value: f64) {
        self.ratings.insert(key, self.ratings.get(&key).unwrap() + value);
    }

    fn mult(&self, key: String, value: f64) {
        self.ratings.insert(key, self.ratings.get(&key).unwrap() * value);
    }

    fn get(&self, key: &str) -> Option<&f64> {
        self.ratings.get(&key.to_string())
    }
}

struct PlayerCompositeRating {
    ratings: HashMap<String, f64>
}

impl PlayerCompositeRating {
    fn insert(&self, key: String, value: f64) {
        self.ratings.insert(key, value);
    }

    fn add(&self, key: String, value: f64) {
        self.ratings.insert(key, self.ratings.get(&key).unwrap() + value);
    }

    fn get(&self, key: &str) -> Option<&f64> {
        self.ratings.get(&key.to_string())
    }
}

struct Injury {
    injury: PlayerInjury,
    playing_through: bool
}

struct PlayerGameSim {
    id: i32,
    name: String,
    age: f64,
    pos: String,
    value_no_pot: f64,
    stat: Stat,
    composite_rating: PlayerCompositeRating,
    skills: Vec<String>,
    injured: bool,
    new_injury: bool,
    injury: Injury,
    pt_modifier: f64
}
struct LastScoringPlay {
    team: i32,
    player: i32,
    shot_type: ShotType,
    time: f32
}

struct ClutchPlay {
    text: String,
    show_notification: bool,
    pids: Vec<i32>,
    tids: Vec<i32>
}

struct TeamStat {
    ptsQtrs: Vec<u8>,
    pts: i32,
}
struct TeamGameSim {
    id: i32,
    pace: f32,
    stat: TeamStat,
    composite_rating: TeamCompositeRating,
    player: Vec<PlayerGameSim>,
    synergy: Synergy
}

impl TeamGameSim {
    fn insert(&mut self, key: String, value: f64) {
        self.composite_rating.insert(key, value);
    }
}

const TEAM_NUMS: [TeamNum; 2] = [0, 1];

fn pick_player(mut ratios:Vec<f32>,
               exempt:Option<PlayerNumOnCourt>) -> i32 {
                if (exempt.is_some()) {
                    ratios[exempt.unwrap() as usize] = 0.0;
                }
                let mut sum = 0.0;
                for ratio in ratios.iter() {
                    sum += ratio;
                }

                if (sum == 0.0) {
                    let mut candidates: Vec<PlayerNumOnCourt> = Vec::new();
                    for (i, ratio) in ratios.iter().enumerate() {
                        if ratio > &0.0 {
                            candidates.push(i as PlayerNumOnCourt);
                        }
                    }
                    let index = rand::thread_rng().gen_range(0..candidates.len());
                    return candidates[index];
                }

                let mut rand = rand::thread_rng().gen_range(0.0..sum);
                let mut running_sum = 0.0;

                for i in 0..ratios.len() {
                    running_sum += ratios[i];
                    if (rand < running_sum) {
                        return i as i32;
                    }
                }

                return 0;

               }

fn get_sorted_indexes(ovrs: Vec<f64>) -> Vec<i32> {
    let mut ovrsSortedDesc = ovrs.clone();
    ovrsSortedDesc.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut usedIndexes: HashSet<i32> = HashSet::new();
    let mut sorted_indexes: Vec<i32> = ovrsSortedDesc.iter().map(|ovr| {
        let mut index = ovrs.iter().position(|x| x == ovr).unwrap() as i32;
        while (usedIndexes.contains(&index)) {
            index += 1;
        }
        usedIndexes.insert(index);
        return index;
    }).collect();
    sorted_indexes.reverse();
    return sorted_indexes;
}

fn boundProb(prob: f64) -> f64 {
    helpers::bound(prob, 0.0, 1.0)
}

struct SkillsCount {
    three: f64,
    A: f64,
    B: f64,
    Di: f64,
    Dp: f64,
    Po: f64,
    Ps: f64,
    R: f64
}

impl SkillsCount {
    pub fn new() -> Self {
        SkillsCount {
            three: 0.0,
            A: 0.0,
            B: 0.0,
            Di: 0.0,
            Dp: 0.0,
            Po: 0.0,
            Ps: 0.0,
            R: 0.0
        }
    }
}

struct GameSim {
    id: i32,
    day: Option<i32>,
    team: [TeamGameSim; 2],
    playersOnCourt: [Vec<i32>; 2],
    startersRecorded: bool,
    subsEveryN: i32,
    overtimes: i32,
    t: i32,
    numPeriods: i32,
    foulsThisQuarter: [i32; 2],
    foulsLastTwoMinutes: [i32; 2],
    averagePossessionLength: f32,
    synergyFactor: f64,
    last_scoring_play: Vec<LastScoringPlay>,
    clutch_plays: Vec<ClutchPlay>,
    o: Option<TeamNum>,
    d: Option<TeamNum>,
    play_by_play: Option<Vec<Play>>,
    allStarGame: bool,
    elam: bool,
    elamActive: bool,
    elamDone: bool,
    elamTarget: i32,
    fatigueFactor: f64,
    numPlayersOnCourt: i32,
    baseInjuryRate: f64,
    gender: String
}

impl GameSim {
    pub fn new(mut self,
               gid: i32, 
               day: Option<i32>,
               teams: [TeamGameSim; 2],
               doPlayByPlay: Option<bool>,
               homeCourtFactor: Option<f32>,
               allStarGame: Option<bool>,
               baseInjuryRate: f64,
               disableHomeCourtAdvantage: Option<bool>) -> Self {
                let g = GameAttributes::new(None);
                let mut play_by_play: Option<Vec<Play>> = None;
                if doPlayByPlay.is_some() {
                    if doPlayByPlay.unwrap() {
                        play_by_play = Some(Vec::new());
                    }
                }
                self.id = gid;
                self.day = day;
                self.team = teams;
                self.baseInjuryRate = baseInjuryRate;
                self.play_by_play = play_by_play;

                let numPlayersOnCourt = g.leagueWithHistory.as_ref().unwrap().game_attributes_league.numPlayersOnCourt;

                self.numPlayersOnCourt = numPlayersOnCourt;
                let mut playersOnCourt = [Vec::new(), Vec::new()];
                for i in 0..self.numPlayersOnCourt {
                    playersOnCourt[0].push(i);
                    playersOnCourt[1].push(i);
                }
                self.playersOnCourt = playersOnCourt;

                self.startersRecorded = false;

                self.updatePlayersOnCourt(&g, None);
                self.updateSynergy();

                return self;


               }

    fn updatePlayersOnCourt(&self, g:&GameAttributes, shooter:Option<PlayerNumOnCourt>) -> bool {
        let substitutions = false;
        let mut blowout = false;
        let lateGame = self.isLateGame();

        let foulsNeededToFoulOut = g.leagueWithHistory.as_ref().unwrap().game_attributes_league.foulsNeededToFoulOut;

        if self.o != None && self.d != None {
            let o = self.o.as_ref().unwrap();
            let o = *o as usize;
            let d = self.d.as_ref().unwrap();
            let d = *d as usize;
            let diff = (&(self.team[d].stat.pts)
            - &(self.team[o].stat.pts))
            .abs();

            let quarter = self.team[o].stat.ptsQtrs.len();
            if self.elamActive {
                let ptsToTarget = 
                    self.elamTarget -
                    cmp::max(self.team[d].stat.pts, self.team[o].stat.pts);
                blowout = diff >= 20 && ptsToTarget < diff;
            } else {
                blowout =
                    quarter == self.numPeriods as usize &&
                    ((diff >= 30 && self.t < 12) ||
                        (diff >= 25 && self.t < 9) ||
                        (diff >= 20 && self.t < 7) ||
                        (diff >= 15 && self.t < 3) ||
                        (diff >= 10 && self.t < 1));
            }
        }

        let foulLimit = self.getFoulTroubleLimit(g);

        for t in TEAM_NUMS.to_vec() {
            let t = t as usize;
            let getOvrs = |includeFouledOut: bool| {
                let ovrs: Vec<f64>;

                for p in 0..self.team[t].player.len() {
                    if self.team[t].player[p].injured ||
                        (!includeFouledOut &&
                            foulsNeededToFoulOut > 0 &&
                        self.team[t].player[p].stat.pf as i32 >= foulsNeededToFoulOut) {
                        ovrs[p] = f64::NEG_INFINITY;
                    } else {
                        let mut rng = rand::thread_rng();
                        ovrs[p] =
                            self.team[t].player[p].value_no_pot *
                            self.fatigue(self.team[t].player[p].stat.energy) *
                            if !lateGame {
                                rng.gen_range(0.9..1.1)
                            } else {
                                1.0
                            };

                        if !self.allStarGame {
                            ovrs[p] *= self.team[t].player[p].pt_modifier;
                        }

                        if blowout {
                            ovrs[p] *= (p as f64 + 1.0) / 10.0;
                        } else {
                            let foulTroubleFactor = self.getFoulTroubleFactor(
                                self.team[t].player[p],
                                foulLimit,
                            );
                            ovrs[p] *= foulTroubleFactor;
                        }

                    }
                }

                return ovrs;
            };

            let numEligiblePlayers = |ovrs: Vec<f64>| {
                let count = 0;
                for ovr in ovrs {
                    if ovr > f64::NEG_INFINITY {
                        count += 1;
                    }
                }

                return count;
            };

            let ovrs = getOvrs(false);

            if numEligiblePlayers(ovrs) < self.numPlayersOnCourt {
                ovrs = getOvrs(true);
            }

            let ovrsOnCourt:Vec<f64> = self.playersOnCourt[t]
                .iter()
                .map(|p| ovrs[*p as usize])
                .collect();

            for pp in get_sorted_indexes(ovrsOnCourt) {
                let pp = pp as usize;
                let p = self.playersOnCourt[t][pp] as usize;
                let onCourtIsIneligible = ovrs[p] == f64::NEG_INFINITY;
                self.playersOnCourt[t][pp] = p as i32;

                if t as i32 == self.o.unwrap() && pp as i32 == shooter.unwrap() {
                    continue;
                }

                for b in 0..self.team[t].player.len() {
                    let b = b as i32;
                    if self.playersOnCourt[t].contains(&b) {
                        continue;
                    }

                    let benchIsValidAndBetter =
                        self.team[t].player[p].stat.courtTime > 2.0 &&
                        self.team[t].player[b as usize].stat.courtTime > 2.0 &&
                        ovrs[b as usize] > ovrs[p];
                    let benchIsEligible =
                        ovrs[b as usize] != f64::NEG_INFINITY;

                    if benchIsValidAndBetter ||
                        (onCourtIsIneligible && benchIsEligible)
                        {
                        let pos:Vec<String> = Vec::new();

                        for j in 0..self.playersOnCourt[t].len() {
                            if (j != pp) {
                                pos.push(self.team[t].player[self.playersOnCourt[t][j] as usize].pos);
                            }
                        }

                        pos.push(self.team[t].player[b as usize].pos);

                        // Requre 2 Gs (or 1 PG) and 2 Fs (or 1 C)
                        let numG = 0;
                        let numPG = 0;
                        let numF = 0;
                        let numC = 0;

                        for j in 0..pos.len() {
                            if (pos[j].contains("G")) {
                                numG += 1;
                            }
                            if (pos[j].contains("PG")) {
                                numPG += 1;
                            }
                            if (pos[j].contains("F")) {
                                numF += 1;
                            }
                            if (pos[j].contains("C")) {
                                numC += 1;
                            }
                        }

                        let cutoff =
                            if self.numPlayersOnCourt >= 5 {
                                2
                            } else {
                                if self.numPlayersOnCourt >= 3 {
                                    1
                                } else {
                                    0
                                }
                            };
                        
                        if (numG < cutoff && numPG == 0) ||
                            (numF < cutoff && numC == 0) {
                                if (
                                    self.fatigue(self.team[t].player[p].stat.energy > 0.728 &&
                                    !onCourtIsIneligible)) {
                                        continue;
                                    }
                            }
                        
                        substitutions = true;

                        self.playersOnCourt[t][pp] = b;
                        self.team[t].player[b as usize].stat.courtTime = rng.gen_range(-2.0..2.0);
                        self.team[t].player[b as usize].stat.benchTime = rng.gen_range(-2.0..2.0);
                        self.team[t].player[p].stat.courtTime = rng.gen_range(-2.0..2.0);
                        self.team[t].player[p].stat.benchTime = rng.gen_range(-2.0..2.0);

                        if self.play_by_play.is_some() {
                            let play = Play::new(
                                PlayType::Sub,
                                t as i32,
                                self.team[t].player[b as usize].id,
                                self.team[t].player[p].id,
                            );
                            self.play_by_play.unwrap().push(play);
                        }

                        if (self.startersRecorded) {
                            self.recordPlay("sub", t, [
                                &self.team[t].player[b as usize].name,
                                &self.team[t].player[p].name,
                            ]);
                        }

                        break;
                    }
                }
            }

        }

        if (!self.startersRecorded) {
            for t in TEAM_NUMS.to_vec() {
                for p in 0..self.team[t as usize].player.len() {
                    let p = p as i32;
                    if self.playersOnCourt[t as usize].contains(&p) {
                        self.recordStat(t, p, "gs");
                    }
                }
            }

            self.startersRecorded = true;
        }

        return substitutions;


    }

    fn getFoulTroubleLimit(&self, g:&GameAttributes) -> i32 {
        let foulsNeededToFoulOut = g.leagueWithHistory.as_ref().unwrap().game_attributes_league.foulsNeededToFoulOut;

        let quarter = self.team[0].stat.ptsQtrs.len();
        if (
            self.overtimes > 0 ||
            self.elamActive ||
            (quarter == self.numPeriods.try_into().unwrap() && self.t < 8)
        ) {
            return foulsNeededToFoulOut;
        }

        let quarterLength = g.leagueWithHistory.as_ref().unwrap().game_attributes_league.quarterLength;

        let gameCompletionFraction =
            (quarter as f64 - self.t as f64 / quarterLength) / self.numPeriods as f64;

        let mut foulLimit = (gameCompletionFraction * foulsNeededToFoulOut as f64).ceil() as i32;

        if (foulLimit < 2) {
            foulLimit = 2;
        } else if (foulLimit >= foulsNeededToFoulOut) {
            foulLimit = foulsNeededToFoulOut - 1;
        }

        return foulLimit;
    }

    fn isLateGame(&self) -> bool {
        let quarter = self.team[0].stat.ptsQtrs.len();
        let lateGame:bool;
        if (self.elamActive) {
            let ptsToTarget = 
                self.elamTarget -
                cmp::max(self.team[self.d.unwrap() as usize].stat.pts, self.team[self.o.unwrap() as usize].stat.pts);
            lateGame = ptsToTarget <= 15;
        } else {
            lateGame = quarter >= self.numPeriods.try_into().unwrap() && self.t < 6;
        }

        return lateGame;
    }

    fn updateSynergy(&mut self) {
        for t in TEAM_NUMS {
            let mut skillsCount = SkillsCount::new();
            for i in 0..self.numPlayersOnCourt {
                let p = self.playersOnCourt[t as usize][i as usize];

                skillsCount.three += helpers::sigmoid(
                    *self.team[t as usize].player[p as usize].composite_rating.get("shootingThreePointer").unwrap(),
                    15.0,
                    0.59
                );
                skillsCount.A += helpers::sigmoid(
                    *self.team[t as usize].player[p as usize].composite_rating.get("athleticism").unwrap(),
                    15.0,
                    0.63
                );
                skillsCount.B += helpers::sigmoid(
                    *self.team[t as usize].player[p as usize].composite_rating.get("dribbling").unwrap(),
                    15.0,
                    0.68
                );
                skillsCount.Di += helpers::sigmoid(
                    *self.team[t as usize].player[p as usize].composite_rating.get("defenseInterior").unwrap(),
                    15.0,
                    0.57
                );
                skillsCount.Dp += helpers::sigmoid(
                    *self.team[t as usize].player[p as usize].composite_rating.get("defensePerimeter").unwrap(),
                    15.0,
                    0.61
                );
                skillsCount.Po += helpers::sigmoid(
                    *self.team[t as usize].player[p as usize].composite_rating.get("shootingLowPost").unwrap(),
                    15.0,
                    0.61
                );
                skillsCount.Ps += helpers::sigmoid(
                    *self.team[t as usize].player[p as usize].composite_rating.get("passing").unwrap(),
                    15.0,
                    0.63
                );
                skillsCount.R += helpers::sigmoid(
                    *self.team[t as usize].player[p as usize].composite_rating.get("rebounding").unwrap(),
                    15.0,
                    0.61
                );
            }

            self.team[t as usize].synergy.off = 0.0;
            self.team[t as usize].synergy.off += 5.0 * helpers::sigmoid(skillsCount.three, 3.0, 2.0);

            self.team[t as usize].synergy.off +=
                3.0 * helpers::sigmoid(skillsCount.B, 15.0, 0.75) +
                helpers::sigmoid(skillsCount.B, 5.0, 1.75);

            self.team[t as usize].synergy.off +=
                3.0 * helpers::sigmoid(skillsCount.Ps, 15.0, 0.75) +
                helpers::sigmoid(skillsCount.Ps, 5.0, 1.75)  +
                helpers::sigmoid(skillsCount.Ps, 5.0, 2.75);
            
            self.team[t as usize].synergy.off += helpers::sigmoid(skillsCount.Po, 15.0, 0.75);

            self.team[t as usize].synergy.off +=
                helpers::sigmoid(skillsCount.A, 15.0, 1.75) +
                helpers::sigmoid(skillsCount.A, 5.0, 2.75);
            
            self.team[t as usize].synergy.off /= 17.0;

            let perim_factor =
                helpers::bound(
                    (1.0 + skillsCount.B + skillsCount.Ps + skillsCount.three).sqrt() - 1.0,
                    0.0,
                    2.0
                ) / 2.0;

            self.team[t as usize].synergy.off *= 0.5 + 0.5 * perim_factor;

            self.team[t as usize].synergy.def = 0.0;
            self.team[t as usize].synergy.def += helpers::sigmoid(skillsCount.Dp, 15.0, 0.75);
            self.team[t as usize].synergy.def += 2.0 * helpers::sigmoid(skillsCount.Di, 15.0, 0.75);
            
            self.team[t as usize].synergy.def +=
                helpers::sigmoid(skillsCount.A, 5.0, 2.0) +
                helpers::sigmoid(skillsCount.A, 5.0, 3.25);

            self.team[t as usize].synergy.def /= 6.0;

            self.team[t as usize].synergy.reb = 0.0;
            self.team[t as usize].synergy.reb +=
                helpers::sigmoid(skillsCount.R, 15.0, 0.75) +
                helpers::sigmoid(skillsCount.R, 5.0, 1.75);

            self.team[t as usize].synergy.reb /= 4.0;

        }
    }

    fn updateTeamCompositeRatings(&self, g:&GameAttributes) {
        let toUpdate = [
            "dribbling",
            "passing",
            "rebounding",
            "defense",
            "defensePerimeter",
            "blocking"
        ];

        let foulLimit = self.getFoulTroubleLimit(g);

        for k in 0..TEAM_NUMS.len() {
            let t = TEAM_NUMS[k] as usize;
            let oppT = TEAM_NUMS[1 - k];
            let diff = self.team[t].stat.pts - self.team[oppT as usize].stat.pts;

            let perfFactor = 1.0 - 0.2 * (diff as f64 / 60.0).tanh();

            for j in 0..toUpdate.len() {
                let rating = toUpdate[j];
                self.team[t].composite_rating.insert(rating.to_string(), 0.0);

                for i in 0..self.numPlayersOnCourt {
                    let p = self.playersOnCourt[t][i as usize];

                    let mut foulLimitFactor = 1.0;
                    if (
                        rating.to_string() == "defense" ||
                        rating.to_string() == "defensePerimeter" ||
                        rating.to_string() == "blocking"
                    ) {
                        let pf = self.team[t].player[p as usize].stat.pf;
                        if (pf as i32 == foulLimit) {
                            foulLimitFactor *= 0.9;
                        } else if (pf as i32 > foulLimit) {
                            foulLimitFactor *= 0.75;
                        }
                    }

                    self.team[t].composite_rating.add(
                        rating.to_string(),
                        self.team[t].player[p as usize].composite_rating.ratings.get(rating).unwrap() *
                        self.fatigue(self.team[t].player[p].stat.energy) *
                        perfFactor *
                        foulLimitFactor
                    );
                }

                self.team[t].composite_rating.mult(rating.to_string(), 1.0/5.0);
            }

            self.team[t].composite_rating.add(
                "dribbling".to_string(),
                self.synergyFactor * self.team[t].synergy.off
            );
            self.team[t].composite_rating.add(
                "passing".to_string(),
                self.synergyFactor * self.team[t].synergy.off
            );
            self.team[t].composite_rating.add(
                "rebounding".to_string(),
                self.synergyFactor * self.team[t].synergy.reb
            );
            self.team[t].composite_rating.add(
                "defense".to_string(),
                self.synergyFactor * self.team[t].synergy.def
            );
            self.team[t].composite_rating.add(
                "defensePerimeter".to_string(),
                self.synergyFactor * self.team[t].synergy.def
            );
            self.team[t].composite_rating.add(
                "blocking".to_string(),
                self.synergyFactor * self.team[t].synergy.def
            );
        }
    }

    fn updatePlayingTime(&self, possessionLength: f64) {
        for t in TEAM_NUMS.to_vec() {
            let t = t as usize;
                for p in 0..self.team[t].player.len() {
                    if self.playersOnCourt[t].contains(p) {
                        self.recordStat(t, p, "min", possessionLength);
                        self.recordStat(t, p, "courtTime", possessionLength);

                        self.recordStat(
                            t,
                            p,
                            "energy",
                            -possessionLength *
                                self.fatigueFactor *
                                (1.0 - self.team[t].player[p].composite_rating.get("endurance").unwrap()),
                        );

                        if self.team[t].player[p].stat.energy < 0.0 {
                            self.team[t].player[p].stat.energy = 0.0;
                        }
                    } else {
                        self.recordStat(t, p, "benchTime", possessionLength);
                        self.recordStat(t, p, "energy", possessionLength * 0.094);

                        if self.team[t].player[p].stat.energy > 1.0 {
                            self.team[t].player[p].stat.energy = 1.0;
                        }
                    }
                }   

        }
    }

    fn injuries(&self, g: &GameAttributes) {
        if g.leagueWithHistory.as_ref().unwrap().game_attributes_league.disableInjuries {
            return;
        }

        let newInjury = false;
        let mut baseRate = self.baseInjuryRate;

        baseRate *= 100.0 / g.leagueWithHistory.as_ref().unwrap().game_attributes_league.pace;

        for t in TEAM_NUMS.to_vec() {
            let t = t as usize;
            for p in 0..self.team[t].player.len() {
                let p = p as i32;
                if self.playersOnCourt[t].contains(&p) {
                    let injuryRate = getInjuryRate(
                        baseRate,
                        self.team[t].player[p as usize].age,
                        Some(self.team[t].player[p as usize].injury.playing_through),
                    );

                    if rng.gen_range(0.0..1.0) < injuryRate {
                        self.team[t].player[p as usize].injured = true;
                        self.team[t].player[p as usize].new_injury = true;
                        newInjury = true;
                        let injuredPID = self.team[t].player[p as usize].id;
                        self.recordPlay("injury",
                            t,
                            [self.team[t].player[p as usize].name],
                            Some(injuredPID),    
                        );
                    }
                }
            }
        }

        if (newInjury) {
            self.updatePlayersOnCourt(g, None);
        }
    }

    fn getNumFoulsUntilBonus(&self, g: &GameAttributes) -> i32 {
        let foulsUntilBonus = g.leagueWithHistory.as_ref().unwrap().game_attributes_league.foulsUntilBonus;
        if (self.t <= 2) {
            return foulsUntilBonus[2] - self.foulsLastTwoMinutes[self.d.unwrap() as usize];
        }
        if (self.overtimes >= 1) {
            return foulsUntilBonus[1] - self.foulsThisQuarter[self.d.unwrap() as usize];
        }
        return foulsUntilBonus[0] - self.foulsThisQuarter[self.d.unwrap() as usize];
    }

    fn getPossessionOutcome(&self, g: &GameAttributes, possessionLength: f64, intentionalFoul: bool) -> String {
        if (
            self.t <= 0 &&
            self.team[self.o.unwrap() as usize].stat.ptsQtrs.len() >= self.numPeriods &&
            self.team[self.o.unwrap() as usize].stat.pts > self.team[self.d.unwrap() as usize].stat.pts &&
            !self.elamActive
        ) {
            return "endOfQuarter".to_string();
        }

        if self.t <= 0 && possessionLength < 6.0 / 60.0 && !self.elamActive {
            if (rng.gen_range(0.0..1.0) > (possessionLength / (8.0 / 60.0)).powf(1.0/4.0)) {
                return "endOfQuarter".to_string();
            }
        }

        if (rng.gen_range(0.0..1.0) < self.probTov(g)) {
            return self.doTov();
        }

        let ratios = self.ratingArray("usage", self.o.unwrap(), 1.25);
        let shooter = pick_player(ratios, None);

        if (rng.gen_range(0.0..1.0) < 0.08 * g.leagueWithHistory.as_ref().unwrap().game_attributes_league.foulRateFactor ||
            intentionalFoul) {
            let numFoulsUntilBonus = self.getNumFoulsUntilBonus(g);
            let inBonus = numFoulsUntilBonus <= 1;

            if (inBonus) {
                self.doPf(self.d.unwrap(), "pfBonus", shooter);
            } else {
                self.doPf(self.d.unwrap(), "pfNonShooting");
            }

            if (inBonus) {
                return self.doFt(shooter, 2);
            }

            return "nonShootingFoul";
        }

        return self.doShot(shooter, possessionLength);
    }

    fn probTov(&self, g: &GameAttributes) -> f64 {
        return boundProb(
            (g.leagueWithHistory.as_ref().unwrap().game_attributes_league.turnoverFactor) *
                (0.14 * self.team[self.d.unwrap() as usize].composite_rating.get("defense").unwrap()) /
                (0.5 *
                    self.team[self.o.unwrap() as usize].composite_rating.get("dribbling").unwrap() +
                        self.team[self.o.unwrap() as usize].composite_rating.get("passing").unwrap()),
        );
    }

    fn doTov(&self) -> String {
        let ratios = self.ratingArray("turnovers", self.)
    }
}