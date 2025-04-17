use std::{collections::HashMap, process::Command, str::FromStr};

use axum::{extract::Query, http::StatusCode};

enum Input {
    Select,
    Start,
    A,
    B,
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "select" => Self::Select,
            "start" => Self::Start,
            "a" => Self::A,
            "b" => Self::B,
            "up" => Self::Up,
            "right" => Self::Right,
            "down" => Self::Down,
            "left" => Self::Left,
            _ => return Err(()),
        })
    }
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

pub async fn sendkey(Query(params): Query<HashMap<String, String>>) -> (StatusCode, &'static str) {
    let Some(Ok(key)) = params.get("key").map(|key| Input::from_str(key)) else {
        return (StatusCode::BAD_REQUEST, "Missing parameter key");
    };

    // We repeat the input a few times to ensure it is catched by the emulator.
    // The game is slow anyway, so it should not cause an issue.
    for _ in 0..3 {
        Command::new("xdotool")
            .arg("key")
            .arg(key.to_keycode())
            .status()
            .unwrap();
    }

    (StatusCode::OK, "OK")
}
