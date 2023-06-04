use core::time::Duration;

use rtc::Rtc;

pub struct Time;

impl Time {
    pub fn now() -> Duration {
        unsafe { Rtc::default().now() }
    }
}
