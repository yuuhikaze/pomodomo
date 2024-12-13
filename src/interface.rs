slint::include_modules!();

use crate::{
    alert::Alert,
    controller,
    pomodoro::PomodoroStatus,
};
use slint::{SharedString, Timer, TimerMode};

pub struct InterfaceBuilder {
    app: AppWindow,
}

impl InterfaceBuilder {
    pub async fn build() -> Self {
        Self {
            app: AppWindow::new().unwrap(),
        }
    }

    pub async fn spawn_and_run(&self) {
        let _timer = self.run_timer().await; // <https://github.com/slint-ui/slint/issues/2482#issuecomment-1500144196>
        self.manage_interface_related_callbacks().await;
        self.app.run().unwrap();
    }

    async fn manage_interface_related_callbacks(&self) {
        let app_weak = self.app.as_weak();
        let app = app_weak.unwrap();
        self.app.on_upgrade(move || {
            Self::set_time(&app, 15 * 60 - app.get_elapsed_time());
            Self::set_status(&app, PomodoroStatus::LongBreak);
        });
    }

    async fn run_timer(&self) -> Timer {
        let app_weak = self.app.as_weak();
        let app = app_weak.unwrap();
        let timer = Timer::default();
        timer.start(
            TimerMode::Repeated,
            std::time::Duration::from_secs(1),
            move || {
                let session = app.get_session();
                let remaining_time = app.get_remaining_time();
                let paused = app.get_paused();
                if !paused {
                    if remaining_time == 0 {
                        app.invoke_complete_session();
                        if session % 2 == 0 {
                            Self::set_time(&app, 5 * 60);
                            Self::set_status(&app, PomodoroStatus::ShortBreak);
                        } else {
                            Self::set_time(&app, 25 * 60);
                            Self::set_status(&app, PomodoroStatus::Focus);
                        }
                    } else {
                        Self::set_time(&app, remaining_time - 1);
                    }
                }
            },
        );
        timer
    }

    fn set_time(app: &AppWindow, seconds: i32) {
        app.set_timer(SharedString::from(format!(
            "{:2}:{:02}",
            seconds / 60,
            seconds % 60
        )));
        app.set_remaining_time(seconds);
    }

    fn set_status(app: &AppWindow, status: PomodoroStatus) {
        app.set_status(SharedString::from(status.to_string()));
        let alert = Alert::new(status);
        alert.notify();
        controller::get_runtime_handle().spawn(async move {
            alert.play_sound().await;
        });
    }
}
