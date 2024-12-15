slint::include_modules!();

use crate::{
    alert::{self, Alert},
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
            Self::set_pt_time(&app, 15 * 60 - app.get_pt_elapsed_time());
            Self::set_pt_status(&app, PomodoroStatus::LongBreak);
        });
    }

    async fn run_timer(&self) -> Timer {
        let app_weak = self.app.as_weak();
        let app = app_weak.unwrap();
        let slint_timer = Timer::default();
        let play_tick_sound = || {
            controller::get_runtime_handle().spawn(async move {
                alert::play_sound("audio/tick.mp3").await;
            });
        };
        slint_timer.start(
            TimerMode::Repeated,
            std::time::Duration::from_secs(1),
            move || {
                let pt_paused = app.get_pt_paused();
                if !pt_paused {
                    let pt_session = app.get_pt_current_session();
                    let pt_remaining_time = app.get_pt_remaining_time();
                    if pt_remaining_time == 0 {
                        app.invoke_complete_pt_session();
                        if pt_session % 2 == 0 {
                            Self::set_pt_time(&app, 5 * 60);
                            Self::set_pt_status(&app, PomodoroStatus::ShortBreak);
                        } else {
                            Self::set_pt_time(&app, 25 * 60);
                            Self::set_pt_status(&app, PomodoroStatus::Focus);
                        }
                    } else {
                        Self::set_pt_time(&app, pt_remaining_time - 1);
                        if pt_remaining_time - 1 <= 15 && pt_remaining_time - 1 > 0 {
                            play_tick_sound();
                        }
                    }
                }
                let ct_paused = app.get_ct_paused();
                if !ct_paused {
                    let ct_remaining_time = app.get_ct_remaining_time();
                    if ct_remaining_time > 0 {
                        Self::set_ct_time(&app, ct_remaining_time - 1);
                        if ct_remaining_time - 1 == 0 {
                            alert::notify("Custom timer", "Finished!");
                            // TODO play unique sound
                            app.set_ct_paused(true);
                        } else if ct_remaining_time - 1 <= 15 {
                            play_tick_sound();
                        }
                    }
                }
            },
        );
        slint_timer
    }

    fn set_pt_time(app: &AppWindow, seconds: i32) {
        app.set_pt_banner(SharedString::from(format!(
            "{:2}:{:02}",
            seconds / 60,
            seconds % 60
        )));
        app.set_pt_remaining_time(seconds);
    }

    fn set_ct_time(app: &AppWindow, seconds: i32) {
        app.set_ct_minutes_banner(SharedString::from(format!("{}", seconds / 60)));
        app.set_ct_seconds_banner(SharedString::from(format!("{:02}", seconds % 60)));
        app.set_ct_remaining_time(seconds);
    }

    fn set_pt_status(app: &AppWindow, pt_status: PomodoroStatus) {
        app.set_pt_status(SharedString::from(pt_status.to_string()));
        let alert = Alert::new(pt_status);
        alert.notify();
        controller::get_runtime_handle().spawn(async move {
            alert.play_sound().await;
        });
    }
}
