use std::env;

pub struct Secrets {
    pub client_id: String,
    pub client_secret: String,
}

impl Secrets {
    pub fn new() -> Secrets {
        Secrets {
            client_id: env::var("GOOGLE_CLIENT_ID").unwrap(),
            client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap(),
        }
    }
}
