#[path = "input_int.rs"] mod input_int;
#[path = "sjf.rs"] mod sjf;
#[path = "lru.rs"] mod lru;
fn main() {
    
    println!("Enter 0 for SJF 1 for LRU");
    let choice =input_int::input_int();
    if choice==1 {
        lru::run_lru();
    }
    else{
        sjf::run_sjf();
    }
}
