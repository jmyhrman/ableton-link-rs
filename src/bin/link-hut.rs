use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

extern crate ableton_link;
use ableton_link::Link;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let quantum = 4.0;
    let mut link = Link::new(120.0);
    let clock = link.clock();
    link.enable(true);

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        link.with_app_session_state(|session_state| {
            let time = clock.micros();
            let tempo = session_state.tempo();
            let playing = session_state.is_playing();
            let beat = session_state.beat_at_time(time, quantum);
            println!(
                "playing={}, quantum={}, clock={}, tempo={}, beat={}",
                playing, quantum, time, tempo, beat
            );
            sleep(Duration::from_millis(100));
        });
    }

    println!("Leaving Link session");

    link.enable(false);
}
