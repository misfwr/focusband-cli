use std::time::{Duration, Instant};
use crate::features::pomodoro::pomodoro_data::Pomodoro;

pub async fn start_pomodoro(pomodoro: &mut Pomodoro){
    pomodoro.is_active = true;

    let now = Instant::now();
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    while pomodoro.is_active {
        interval.tick().await;
        println!("{:?}", now.elapsed().as_secs());
        if now.elapsed().as_secs() >= pomodoro.focus_time.as_secs() {
            pomodoro.stop()
        }
    }
}