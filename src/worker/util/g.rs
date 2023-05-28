use crate::common::types::GameAttributes;
use crate::common::types::GameAttributesLeague;
use crate::common::types::GameAttributesLeagueWithHistory;

pub struct G {
    pub game_attributes: GameAttributes
}

impl G {
    pub fn new(attributes:GameAttributes) -> Self {
        return G {
            game_attributes: attributes
        };
    }
}