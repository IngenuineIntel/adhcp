// adhcp
// thread.rs

use std::{fs, path, process, thread, time};
use std::io::{self, BufRead, BufReader};
use std::sync::mpsc;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

/* Thread 1: The Collection Thread
 * The collection thread is responsible for sending log and lease
 * information to the main thread. It gets the logs from a
 * `journalctl` child process, and calculates the leases via TODO.
 */

pub fn start_collection_thread() -> (mpsc::Receiver<String>, mpsc::Receiver<String>) {
    let (log_sender, log_receiver) = mpsc::channel();
    let (lease_sender, lease_receiver) = mpsc::channel();

    thread::spawn(move || {
        collection_thread(log_sender, lease_sender);
    });

    (log_receiver, lease_receiver)
}

fn collection_thread(log_sender: mpsc::Sender<String>, lease_sender: mpsc::Sender<String>) {

    const COLLECTION_REFRESH_RATE: u8 = 1;
    let cap_duration = time::Duration::from_millis(1000/(COLLECTION_REFRESH_RATE as u64));

    let mut leases_data = String::new();
    let leases_path = path::Path::new("/var/lib/dhcpd/dhcpd.leases");

    loop {

        let fd = fs::File::open("/var/lib/dhcpd/dhcpd.leases");

        let mut cmd = process::Command::new("journalctl")
            .args(&["-fu", "dhcpd", "--no-pager"])
            .stdout(process::Stdio::piped())
            .spawn()
            .expect("Failed to execute journalctl");

        let stdout = cmd.stdout.take().expect("Failed to capture stdout");
        let reader = BufReader::new(stdout);

        // TODO check expirations and shit
        
        for line in reader.lines() {
            if let Ok(line) = line {
               let _ = log_sender.send(line.clone());
            }
        }
        
        thread::sleep(cap_duration);
    }
}

/* Thread 2: The Input Thread
 * The input thread is responsible for sending input information to
 * the main thread. It uses the `crossterm` backend to do so.
 */

pub fn start_input_thread() -> mpsc::Receiver<KeyEvent> {
    let (input_sender, input_receiver) = mpsc::channel();

    thread::spawn(move || {
        input_thread(input_sender);
    });

    input_receiver
}

fn input_thread(input_sender: mpsc::Sender<KeyEvent>) {
    loop {
        if event::poll(time::Duration::from_millis(250)).expect("") {
            if let Ok(Event::Key(key)) = event::read() {
                if key.kind == event::KeyEventKind::Press {
                    let _ = input_sender.send(key.clone());
                }
            }
        }
    }
}

