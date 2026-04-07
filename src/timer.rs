use esp_hal::{Blocking, time::Duration, timer::PeriodicTimer};

pub fn start_timer(mut timer: PeriodicTimer<'static, Blocking>) {
    timer
        .start(Duration::from_secs(1))
        .expect("Failed to start timer");
    timer.listen();
}
