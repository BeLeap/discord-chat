use std::env;

use super::chattable::Chattable;

pub struct Bard {
    token: String,
}

impl Bard {
    pub fn new(token: Option<String>) -> Bard {
        Bard {
            token: match token {
                Some(token) => token,
                None => match env::var("BARD_API_TOKEN") {
                    Ok(token) => token,
                    Err(e) => panic!("Bard api token not provided: {:#?}", e),
                },
            },
        }
    }
}

impl Chattable for Bard {
    fn chat(&self, instruction: String) -> String {
        format!("Response for {}", instruction).to_string()
    }
}
