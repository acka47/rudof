
use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

#[derive(Default, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub struct Lang {
    lang: String
}

impl Display for Lang {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.lang)
    }
}
