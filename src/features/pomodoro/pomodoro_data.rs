use std::time::Duration;
use crate::start_pomodoro;

pub struct Pomodoro {
    pub focus_time: Duration,
    pub relax_time: Duration,
    pub actual_time: Option<Duration>,
    pub has_large_relax: bool,
    pub large_relax_time: Option<Duration>,
    pub is_active: bool,
    pub is_paused: bool,
    pub remaining_time: Option<Duration>
}

impl Pomodoro {
    pub fn new() -> Self{
        Self{
            focus_time: Duration::from_secs(25*60),
            relax_time: Duration::from_secs(5*60),
            actual_time: None,
            has_large_relax: true,
            large_relax_time: Option::from(Duration::from_secs(15 * 60)),
            is_active: false,
            is_paused: false,
            remaining_time: None
        }
    }

    pub async fn start(&mut self) {
        start_pomodoro(self).await;
    }
    pub fn stop(&mut self) {
        self.is_active = false;
    }
    pub fn pause(&mut self) {}
    pub fn restart(&mut self) {
        self.stop();
        self.start();
    }
    pub fn skip(&mut self) {}

}