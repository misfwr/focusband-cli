use std::time::Duration;
use crate::features::pomodoro::pomodoro_data::Pomodoro;
use crate::features::pomodoro::pomodoro_handler::start_pomodoro;

mod features;

#[tokio::main]
async fn main() {
    let option1 = std::env::args().nth(1).expect("please supply an argument");
    if option1.starts_with("hola") {
        println!("Hello, world!");
    } else if option1.starts_with("pomodoro") {
        let mut pomodoro: Pomodoro = Pomodoro::new();
        pomodoro.focus_time = Duration::from_secs(20);
        pomodoro.start().await
    }
}
