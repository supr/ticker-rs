extern crate ticker;

fn main() {
    let t = ticker::Ticker::new(1000u32);
    t.arm();
    for _ in t {
        println!("Ticked!");
    }
}
