use crate::core::player::Player;
use crate::core::uniq::GeneratorUnicID;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::{Arc, RwLock};
use std::thread::sleep_ms;
use std::thread::spawn;
use std::time::Instant;
use std::time::SystemTime;
use tungstenite::{accept, Message};

#[derive(Serialize, Deserialize)]
pub enum SignType {
    SignIn,
    SignUp,
}

#[derive(Serialize, Deserialize)]
pub struct Sign {
    #[serde(rename = "type")]
    pub typ: SignType,
    pub username: String,
    pub password: String,
}

type Websocket = tungstenite::WebSocket<std::net::TcpStream>;
type Users = Arc<RwLock<HashMap<(String, String), Player>>>;

type AuthError = ();

pub struct Server {}

impl Server {
    pub fn run(&mut self) {
        let users: Users = Arc::new(RwLock::new(HashMap::new()));
        let generator_id: Arc<RwLock<GeneratorUnicID>> =
            Arc::new(RwLock::new(GeneratorUnicID::new()));
        let (sender, receiver) = mpsc::channel::<Websocket>();

        let server = TcpListener::bind("127.0.0.1:3030").unwrap();
        for stream in server.incoming() {
            let stream = stream.unwrap();
            let mut websocket = accept(stream).unwrap();
            let users = Arc::clone(&users);
            let generator_id = Arc::clone(&generator_id);
            spawn(move || {
                while Server::auth(&mut websocket, &users, &generator_id) != Ok(()) {}
                Server::handle(&mut websocket, &users, &generator_id);
            });
        }
    }

    fn auth(
        websocket: &mut Websocket,
        users: &Users,
        generator_id: &Arc<RwLock<GeneratorUnicID>>,
    ) -> Result<(), AuthError> {
        let request = websocket.read_message().unwrap_or(Message::from(""));
        let text = request.into_text().unwrap_or_default();
        match serde_json::from_str::<Sign>(text.as_str()) {
            Ok(sign) => match sign.typ {
                SignType::SignIn => {
                    let mut users = users.write().unwrap();
                    match users.get(&(sign.username, sign.password)) {
                        Some(value) => {
                            websocket.write_message(Message::from("Ok")).unwrap();
                            return Ok(());
                        }
                        None => {
                            websocket
                                .write_message(Message::from("Account not found"))
                                .unwrap();
                            return Err(());
                        }
                    }
                }
                SignType::SignUp => {
                    let mut users = users.write().unwrap();
                    match users.get(&(sign.username.clone(), sign.password.clone())) {
                        Some(_) => {
                            websocket
                                .write_message(Message::from("This user yet registers"))
                                .unwrap();
                            return Err(());
                        }
                        None => {
                            log::info!("Register: {}, {}", sign.username, sign.password);
                            users.insert(
                                (sign.username, sign.password),
                                Player::new_by_id(generator_id.write().unwrap().new_id()),
                            );
                            websocket.write_message(Message::from("Ok")).unwrap();
                            return Ok(());
                        }
                    }
                }
            },
            Err(err) => {
                return Err(());
            }
        }
    }

    fn handle(
        websocket: &mut Websocket,
        users: &Users,
        generator_id: &Arc<RwLock<GeneratorUnicID>>,
    ) {
        let answer = String::from(
            websocket
                .read_message()
                .unwrap_or(Message::from(""))
                .to_text()
                .unwrap_or_default(),
        );
        websocket.write_message(Message::from(answer)).unwrap();
    }
}
