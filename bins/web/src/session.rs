use actix_web::{
    HttpMessage,
    HttpRequest,
    dev::HttpResponseBuilder,
};
use ring::rand::SecureRandom;
use std::{
    collections::HashMap,
    sync::Mutex,
};

type SessionKey = String;

#[derive(Clone)]
pub struct Session {
    key: String,
}

pub struct Sessions {
    all: Mutex<HashMap<SessionKey, Session>>,
    rng: ring::rand::SystemRandom,
    secure: bool,
}

const SESSION_COOKIE_NAME: &'static str = "monitor-web-session";

impl Sessions {
    pub fn with_secure(secure: bool) -> Sessions {
        Sessions {
            all: Mutex::new(HashMap::new()),
            rng: ring::rand::SystemRandom::new(),
            secure,
        }
    }

    pub fn login(&self, res: &mut HttpResponseBuilder) -> Result<(), String> {
        let mut key_bytes = [0; 16]; // 16 bytes = 128 bits.
        self.rng.fill(&mut key_bytes).map_err(|_| "Failed to generate random key")?;
        let key = base64::encode_config(&key_bytes, base64::URL_SAFE_NO_PAD);

        let session = Session {
            key: key.to_owned(),
        };
        let _ = self.all.lock().unwrap().insert(key.to_owned(), session);
        res.cookie(actix_web::http::CookieBuilder::new(SESSION_COOKIE_NAME, key)
                   .max_age(365 * time::Duration::day()) // ~1 year
                   .secure(self.secure)
                   .http_only(true)
                   .same_site(cookie::SameSite::Lax)
                   .finish());
        Ok(())
    }

    pub fn get_with_req(&self, req: &HttpRequest) -> Option<Session> {
        let cookie = req.cookie(SESSION_COOKIE_NAME)?;
        let val = cookie.value();
        self.get_with_key(val)
    }

    pub fn get_with_key(&self, key: &str) -> Option<Session> {
        self.all.lock().unwrap().get(key).cloned()
    }

    pub fn logout(&self, req: &HttpRequest, res: &mut HttpResponseBuilder) {
        // Delete the session cookie by setting its value to empty and its max-age negative
        res.cookie(actix_web::http::CookieBuilder::new(SESSION_COOKIE_NAME, "")
                   .max_age(-1 * time::Duration::second())
                   .secure(self.secure)
                   .http_only(true)
                   .same_site(cookie::SameSite::Lax)
                   .finish());

        if let Some(s) = self.get_with_req(req) {
            self.all.lock().unwrap().remove(&s.key);
        }

        trace!("logout: sessions.all.len()={}", self.all.lock().unwrap().len());
    }
}
