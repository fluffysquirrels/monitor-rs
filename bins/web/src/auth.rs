use std::{
    collections::HashMap,
    sync::Mutex,
};
use ring::rand::SecureRandom;

pub struct Auth {
    users_by_name: Mutex<HashMap<String, User>>,
    rng: ring::rand::SystemRandom,
}

#[derive(Clone)]
pub struct User {
    pub username: String,
    password_hash: String,
}

pub struct AuthToken {
    username: String,
}

impl Auth {
    pub fn new() -> Auth {
        let mut users = HashMap::<String, User>::new();
        users.insert("alex".to_owned(), User {
            username: "alex".to_owned(),
            password_hash: "$argon2id$v=19$m=65536,t=12,p=1$BDddaZnrMqUwO9FbcNtKug$YN0sqPflohLSb701QD52H8AVZPo827FpmouXZM/0Ix4".to_owned(),
        });
        Auth {
            users_by_name: Mutex::new(users),
            rng: ring::rand::SystemRandom::new(),
        }
    }

    pub fn create_user(&self, username: &str, password: &str) -> Result<User, String> {
        let mut salt = [0; 16]; // 16 bytes = 128 bits.
        self.rng.fill(&mut salt).map_err(|_| "Failed to generate random salt")?;

        // Tuned to about 1s on my laptop.
        let mut config = argon2::Config::default();
        config.variant = argon2::Variant::Argon2id;
        config.time_cost = 12;
        config.mem_cost = 64 * 1024; // 64MB

        let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config)
                          .map_err(|e| format!("argon2::hash_encoded error: {}", e))?;
        let user = User {
            username: username.to_owned(),
            password_hash: hash,
        };
        Ok(user)
    }

    pub fn login(&self, username: &str, password: &str) -> Option<AuthToken> {
        let user = self.users_by_name.lock().unwrap().get(username).cloned();
        match user {
            None => {
                // Verify a dummy hash when user not found to avoid a
                // timing information leakage where missing users
                // return more quickly than valid users.
                if let Err(e) = argon2::verify_encoded("$argon2id$v=19$m=65536,t=12,p=1$eut20+7nNIP/oIknv5lBgw$uQ16nE2xRTz392vJNySeO8pP1uqNcTWc4e/K0r38D2c", password.as_bytes()) {
                    error!("argon2::verify_encoded err: {}", e);
                    return None;
                }
                None
            }
            Some(user) => {
                let ok = match argon2::verify_encoded(&user.password_hash, password.as_bytes()) {
                    Err(e) => {
                        error!("argon2::verify_encoded err: {}", e);
                        return None;
                    },
                    Ok(ok) => ok,
                };
                if ok {
                    Some(AuthToken {
                        username: user.username.clone(),
                    })
                } else {
                    None
                }
            }
        }
    }
}
