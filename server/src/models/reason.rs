use edgedb_tokio::Queryable;
use serde::Deserialize;

#[derive(Debug, Deserialize, Queryable)]
#[serde(rename_all = "snake_case")]
pub enum Reason {
    Cheating,
    Griefing,
    Toxicity,
}

impl Reason {
    pub fn weight(&self) -> usize {
        match self {
            Self::Cheating => 3,
            Self::Griefing => 2,
            Self::Toxicity => 1,
        }
    }
}

impl ToString for Reason {
    fn to_string(&self) -> String {
        // NOTE: this, for now, needs to follow THIS EXACT CASING, because
        //       it is used to pass in Reason values as QueryArgs.
        // TODO: improve once the Rust driver adds Enum QueryArgs support.
        match self {
            Self::Cheating => "Cheating",
            Self::Griefing => "Griefing",
            Self::Toxicity => "Toxicity",
        }
        .to_owned()
    }
}
