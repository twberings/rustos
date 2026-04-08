use core::cell::RefCell;

use critical_section::{CriticalSection, Mutex};
use esp_hal::{
    Blocking,
    interrupt::{InterruptHandler, software::SoftwareInterrupt},
    time::Duration,
    timer::PeriodicTimer,
};

static TIMER: Mutex<RefCell<Option<PeriodicTimer<'static, Blocking>>>> =
    Mutex::new(RefCell::new(None));

pub fn start_timer(mut timer: PeriodicTimer<'static, Blocking>) {
    critical_section::with(|cs| {
        timer
            .start(Duration::from_secs(1))
            .expect("Failed to start timer");
        timer.listen();
        timer.set_interrupt_handler(InterruptHandler::new(
            handler,
            esp_hal::interrupt::Priority::Priority1,
        ));
        TIMER.borrow(cs).replace(Some(timer));
    });
}

#[esp_hal::ram]
extern "C" fn handler() {
    critical_section::with(|cs| {
        if let Some(ref mut timer) = *TIMER.borrow(cs).borrow_mut() {
            timer.clear_interrupt();
        }
    });
    unsafe {
        SoftwareInterrupt::<'static, 0>::steal().raise();
    }
}
