use rand::seq::IteratorRandom;
use rocket::{
    FromFormField, State,
    http::{Cookie, CookieJar, Status},
    post,
};
use std::{
    collections::HashMap,
    process::Command,
    str::FromStr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::{interval, sleep};
use uuid::Uuid;

const SESSION_COOKIE: &str = "icp-session";
const INTERVAL_MILLIS: u64 = 1000;

#[derive(FromFormField, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Input {
    Select,
    Start,
    A,
    B,
    Up,
    Right,
    Down,
    Left,
}

impl Input {
    fn to_keycode(&self) -> &str {
        match self {
            Input::Select => "u",
            Input::Start => "i",
            Input::A => "k",
            Input::B => "j",
            Input::Up => "w",
            Input::Right => "d",
            Input::Down => "s",
            Input::Left => "a",
        }
    }
}

pub type InputState = Arc<Mutex<HashMap<Uuid, Input>>>;

#[post("/sendkey?<key>")]
pub async fn sendkey(
    key: Input,
    state: &State<InputState>,
    cookies: &CookieJar<'_>,
) -> (Status, &'static str) {
    // We repeat the input a few times to ensure it is catched by the emulator.
    // The game is slow anyway, so it should not cause an issue.

    let id = match cookies.get(SESSION_COOKIE) {
        Some(id) => Uuid::from_str(id.value()).unwrap_or_else(|_| Uuid::new_v4()),
        None => Uuid::new_v4(),
    };
    cookies.add(Cookie::build((SESSION_COOKIE, id.to_string())));

    state.lock().unwrap().insert(id, key);

    (Status::Ok, "OK")
}

fn run_input(key: Input) {
    for _ in 0..3 {
        Command::new("xdotool")
            .arg("key")
            .arg(key.to_keycode())
            .status()
            .unwrap();
    }
}

pub async fn background(state: InputState) {
    let now = chrono::offset::Local::now();
    let rem_millis = INTERVAL_MILLIS - (now.timestamp_millis() as u64 % INTERVAL_MILLIS);

    sleep(Duration::from_millis(rem_millis)).await;
    let mut interval = interval(Duration::from_millis(INTERVAL_MILLIS));

    loop {
        interval.tick().await;

        let mut state = state.lock().unwrap();

        let key = state.values().choose(&mut rand::rng()).copied();
        if let Some(key) = key {
            run_input(key);
        }

        state.clear();

        println!("Chosen key: {key:?}")
    }
}
