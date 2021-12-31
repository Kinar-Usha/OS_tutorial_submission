#[path = "input_int.rs"]
mod input_int;
#[path = "lru.rs"]
mod lru;
#[path = "sjf.rs"]
mod sjf;
fn main() {
    println!("Enter 0 for SJF 1 for LRU");
    let choice = input_int::input_int();
    if choice == 1 {
        lru::run_lru();
    } else {
        sjf::run_sjf();
    }
}
