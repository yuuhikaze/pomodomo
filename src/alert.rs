use notify_rust::Notification;
use rodio::{Decoder, OutputStream, Sink};
use rust_embed::Embed;
use std::io::Cursor;

use crate::pomodoro::PomodoroStatus;

#[derive(Embed)]
#[folder = "static/"]
struct Asset;

pub struct Alert {
    pomodoro_status: PomodoroStatus,
}

impl Alert {
    pub fn new(pomodoro_status: PomodoroStatus) -> Self {
        Self { pomodoro_status }
    }

    pub async fn play_sound(&self) {
        let path = match self.pomodoro_status {
            PomodoroStatus::Focus => "audio/focus.mp3",
            PomodoroStatus::ShortBreak => "audio/short-break.mp3",
            PomodoroStatus::LongBreak => "audio/long-break.mp3",
        };
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        if let Some(file) = Asset::get(path) {
            let cursor = Cursor::new(file.data);
            let source = Decoder::new(cursor).unwrap();
            sink.append(source);
            std::thread::sleep(std::time::Duration::from_secs(5));
            sink.sleep_until_end();
        } else {
            eprintln!("Failed to load asset: {}", path);
        }
    }

    pub fn notify(&self) {
        let (summary, body) = match self.pomodoro_status {
            PomodoroStatus::Focus => ("Break Finished", "Begin focusing for 25 minutes"),
            PomodoroStatus::ShortBreak => ("Focus Round Complete", "Begin a 5 minute short break"),
            PomodoroStatus::LongBreak => {
                ("Short Break Upgraded", "Enjoy your 15 minute long break!")
            }
        };
        Notification::new()
            .summary(summary)
            .body(body)
            .show()
            .unwrap();
    }
}