use statemap::Statemap;

use chrono::{DateTime, Utc};
use std::sync::mpsc::channel;
use std::thread;
use std::time;

fn slow() {
    thread::sleep(time::Duration::from_millis(1000));
}

fn faster() {
    thread::sleep(time::Duration::from_millis(200));
}

fn fastest() {
    thread::sleep(time::Duration::from_millis(100));
}

struct State {
    name: String,
    id: String,
    time: DateTime<Utc>,
}

fn main() {
    let mut statemap = Statemap::new("demo statemap", Some("localhost".to_owned()), None);

    let (tx, rx) = channel();

    let t0_tx = tx.clone();
    let t1_tx = tx.clone();

    drop(tx);

    thread::spawn(move || {
        let id = "thread0";
        for _ in 0..5 {
            t0_tx
                .send(State {
                    name: "slow".to_owned(),
                    id: id.to_owned(),
                    time: Utc::now(),
                })
                .unwrap();
            slow();

            t0_tx
                .send(State {
                    name: "faster".to_owned(),
                    id: id.to_owned(),
                    time: Utc::now(),
                })
                .unwrap();
            faster();
        }
    });

    thread::spawn(move || {
        let id = "thread1";
        for _ in 0..20 {
            t1_tx
                .send(State {
                    name: "faster".to_owned(),
                    id: id.to_owned(),
                    time: Utc::now(),
                })
                .unwrap();
            faster();

            t1_tx
                .send(State {
                    name: "fastest".to_owned(),
                    id: id.to_owned(),
                    time: Utc::now(),
                })
                .unwrap();
            fastest();
        }
    });

    while let Ok(s) = rx.recv() {
        statemap.set_state("main", "receiving", None, Utc::now());

        statemap.set_state(&s.id, &s.name, None, s.time);

        statemap.set_state("main", "waiting", None, Utc::now());
    }

    statemap.set_state("main", "creating statemap", None, Utc::now());

    statemap.set_state_color("waiting", "yellow");
    statemap.set_state_color("receiving", "red");
    statemap.set_state_color("slow", "orange");
    statemap.set_state_color("faster", "blue");
    statemap.set_state_color("fastest", "green");

    for state in statemap {
        println!("{}", state);
    }
}
