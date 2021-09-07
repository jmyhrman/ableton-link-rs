extern crate ableton_link;

use ableton_link::Link;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let quantum = 4.0;
    let mut link = Link::new(120.0);
    let clock = link.clock();
    link.enable(true);
    loop {
        link.with_app_session_state(|mut session_state| {
            let time = clock.micros();
            let tempo = session_state.tempo();
            let playing = session_state.is_playing();
            let beat = session_state.beat_at_time(time, quantum);
            println!(
                "playing={}, quantum={}, clock={}, tempo={}, beat={}",
                playing, quantum, time, tempo, beat
            );
            sleep(Duration::from_millis(100));
            //session_state.set_tempo(122.0, 0);
            //session_state.commit();
        });
    }
}
