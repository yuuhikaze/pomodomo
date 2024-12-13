slint::include_modules!();

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
            app.set_remaining_time(15 * 60 - app.get_elapsed_time());
            app.set_status(SharedString::from("Long Break"));
        });
    }

    async fn run_timer(&self) -> Timer {
        let app_weak = self.app.as_weak();
        let app = app_weak.unwrap();
        let set_time = move |seconds| {
            app.set_timer(SharedString::from(format!(
                "{:2}:{:02}",
                seconds / 60,
                seconds % 60
            )));
            app.set_remaining_time(seconds);
        };
        let app = app_weak.unwrap();
        let timer = Timer::default();
        timer.start(
            TimerMode::Repeated,
            std::time::Duration::from_secs(1),
            move || {
                let cycle = app.get_cycle();
                let remaining_time = app.get_remaining_time();
                let paused = app.get_paused();
                if !paused {
                    if remaining_time == 0 {
                        app.invoke_complete_cycle();
                        if cycle % 2 == 0 {
                            set_time(5 * 60);
                            app.set_status(SharedString::from("Short Break"));
                        } else {
                            set_time(25 * 60);
                            app.set_status(SharedString::from("Focus"));
                        }
                    } else {
                        set_time(remaining_time - 1);
                    }
                }
            },
        );
        timer
    }
}
