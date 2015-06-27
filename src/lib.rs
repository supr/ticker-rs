use std::thread;
use std::sync::mpsc;
use std::iter::Iterator;

pub struct Ticker {
    tick_on: u32,
    sender: mpsc::Sender<()>,
    receiver: mpsc::Receiver<()>
}

impl Ticker {
    pub fn new(tick_on: u32) -> Ticker {
        let (sender, receiver) = mpsc::channel::<()>();
        Ticker { tick_on: tick_on, sender: sender, receiver: receiver }
    }

    //TODO: Try to arm the ticker during initialization
    pub fn arm(&self) {
        let tick_on = self.tick_on;
        let tx = self.sender.clone();
        thread::spawn(move|| {
            loop {
                thread::sleep_ms(tick_on);
                tx.send(()).unwrap();
            }
        });
    }
}

impl Iterator for Ticker {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.receiver.recv().is_ok() {
            Some(())
        } else {
            None
        }
    }
}
