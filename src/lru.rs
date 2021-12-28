#[path = "input_int.rs"] mod input_int;
// use std::io::Write;
const ARRAY_LENGTH:usize =20;
// use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
fn print_set(s:&BTreeSet<u32>){
    for i in s.iter(){
        print!("    {}     |",*i);
    }
    println!("");
}
fn page_replacement(pages:&mut [u32], num_of_pages:usize, num_of_frames:usize) {
    // let mut set: HashSet<u32> = HashSet::new();
    let mut set = BTreeSet::new();
    let mut indexes: HashMap<u32, u32> = HashMap::new();
    let mut page_hit:u32 =0;
    let mut page_faults:u32 =0;
    for i in 0..num_of_pages{
        if set.len() < num_of_frames{
            if !set.contains(&pages[i]){
                set.insert(pages[i]);
                page_faults+=1;
            }
            else{
                page_hit+=1;
            }
            


            indexes.insert(pages[i],i as u32);
        }
        else {
            if !set.contains(&pages[i]) {
                let mut lru:u32=u32::MAX;
                let mut val:u32 =0;
                for it in set.iter(){
                    if *indexes.get(&it).unwrap() < lru{
                        lru = *indexes.get(&it).unwrap();
                        val = *it;
                    }
                }
                set.remove(&val);
                set.insert(pages[i]);
                page_faults+=1;
            }
            else{
                page_hit+=1;
            }
            indexes.insert(pages[i],i as u32);
        }
        print_set(&set);
    }
    println!("\nhits = {}\nfaults = {}",page_hit,page_faults);

    
}

pub fn run_lru(){
    println!("Enter the number of pages:");
    let  nop:u32 = input_int::input_int();
    let num_of_pages:usize = nop as usize;
    println!("Enter the Reference String:");
    let mut pages : [u32;ARRAY_LENGTH] =[0;ARRAY_LENGTH];
    for i in 0..num_of_pages{
        pages[i]=input_int::input_int();
    }
    println!("Enter the number of frames:");
    let nof : u32  = input_int::input_int();
    let num_of_frames :usize = nof as usize;
    for i in 0..num_of_frames{
        print!("  Frame {} |",i);
    }
    println!();
    page_replacement(&mut pages, num_of_pages, num_of_frames);
}