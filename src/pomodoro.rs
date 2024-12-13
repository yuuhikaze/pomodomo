use std::fmt::Display;

pub enum PomodoroStatus {
    Focus,
    ShortBreak,
    LongBreak,
}

impl Display for PomodoroStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PomodoroStatus::Focus => write!(f, "Focus"),
            PomodoroStatus::ShortBreak => write!(f, "Short Break"),
            PomodoroStatus::LongBreak => write!(f, "Long Break"),
        }
    }
}
