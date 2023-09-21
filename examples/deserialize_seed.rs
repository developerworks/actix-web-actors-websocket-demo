// use serde::de::{DeserializeSeed, Deserializer};
// use serde::Deserialize;
// use serde_json::Value;
// use std::fmt;

use serde::de::{DeserializeSeed, Deserializer};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, serde::Serialize)]
struct Login {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, serde::Serialize)]
struct UpdateProfile {
    username: String,
    email: String,
    phone: String,
}

#[derive(Debug, serde::Serialize)]
enum MessageType {
    Login(Login),
    UpdateProfile(UpdateProfile),
}

struct MessageSeed;

impl<'de> DeserializeSeed<'de> for MessageSeed {
    type Value = MessageType;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json: Value = Deserialize::deserialize(deserializer)?;

        match json.get("type").and_then(|v| v.as_str()) {
            Some("Login") => {
                // let login: Login = serde_json::from_value(json)?;
                // Ok(MessageType::Login(login))
                let login: Result<Login, _> = serde_json::from_value(json);
                match login {
                    Ok(login) => Ok(MessageType::Login(login)),
                    Err(e) => Err(serde::de::Error::custom(e.to_string())),
                }
            }
            Some("UpdateProfile") => {
                // let update_profile: UpdateProfile = serde_json::from_value(json)?;
                // Ok(MessageType::UpdateProfile(update_profile))
                let update_profile: Result<UpdateProfile, _> = serde_json::from_value(json);
                match update_profile {
                    Ok(update_profile) => Ok(MessageType::UpdateProfile(update_profile)),
                    Err(e) => Err(serde::de::Error::custom(e.to_string())),
                }
            }
            _ => Err(serde::de::Error::custom("unknown message type")),
        }
    }
}

fn main() {
    let json = r#"{"type": "Login", "username": "Alice", "password": "secret"}"#;
    let seed = MessageSeed;
    let mut deserializer = serde_json::Deserializer::from_str(json);
    let message: MessageType = seed.deserialize(&mut deserializer).unwrap();
    let output = serde_json::to_string_pretty(&message).unwrap();
    println!("{}", output);

    let json = r#"{"type": "UpdateProfile", "username": "Alice", "email": "alice@example.com", "phone": "1234567890"}"#;
    let seed = MessageSeed;
    let mut deserializer = serde_json::Deserializer::from_str(json);
    let message: MessageType = seed.deserialize(&mut deserializer).unwrap();
    let output = serde_json::to_string_pretty(&message).unwrap();
    println!("{}", output);
}
