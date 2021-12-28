#[path = "input_int.rs"] mod input_int;
const N: usize = 5;

pub struct ProcessTable {
    id: u32,
    at: u32,
    bt: u32,
    ct: u32,
    tat: u32,
    wt: u32,
}
pub fn sort_arrival(ar: &mut Vec<ProcessTable>) {
    for i in 0..N {
        for j in 0..N - i - 1 {
            if ar[j].at > ar[j + 1].at {
                ar.swap(j, j + 1);
            } else {
                if ar[j].at == ar[j + 1].at {
                    if ar[j].id > ar[j + 1].id {
                        ar.swap(j, j + 1);
                    }
                }
            }
        }
    }
}
pub fn print_sjf(ar: &mut Vec<ProcessTable>) {
    println!("processID| Arrival Time | Burst time\t| Completion time | turn around time\t| waiting time");
    let mut avg_tat:u32 = 0;
    let mut avg_wt:u32 = 0;
    for i in 0..N {
        avg_tat+=ar[i].tat;
        avg_wt+=ar[i].wt;
        
        println!(
            "{}\t |\t{}\t|\t{}\t| \t{}\t  | \t{}\t\t|\t{}",
            ar[i].id, ar[i].at, ar[i].bt, ar[i].ct, ar[i].tat, ar[i].wt
        );
    }
    avg_tat/=N as u32;
    avg_wt/=N as u32;
    println!("Average turn around time = {}\nAverage waiting time = {}",avg_tat,avg_wt);
}

pub fn completion_time(vector: &mut Vec<ProcessTable>) {
    let mut temp: u32;
    vector[0].ct = vector[0].at + vector[0].bt;
    vector[0].tat = vector[0].ct - vector[0].at;
    vector[0].wt = vector[0].tat - vector[0].bt;

    let mut val: usize = 0;
    for i in 1..N {
        temp = vector[i - 1].ct;
        let mut low = vector[i].bt;
        for j in i..N {
            if temp >= vector[j].at && low >= vector[j].bt {
                low = vector[j].bt;
                val = j;
            }
        }
        vector[val].ct = temp + vector[val].bt;
        vector[val].tat = vector[val].ct - vector[val].at;
        vector[val].wt = vector[val].tat - vector[val].bt;

        vector.swap(val, i);
    }
}
pub fn run_sjf(){
    println!("The number of processes: {}", N);
    let mut process_vector_table: Vec<ProcessTable> = Vec::with_capacity(N);
    for _ in 0..N {
        println!("Enter the process ID: ");
        let id = input_int::input_int();
        println!("Enter the Arrival Time: ");
        let at = input_int::input_int();
        println!("Enter the Burst Time: ");
        let bt = input_int::input_int();
        process_vector_table.push(ProcessTable {
            id: id,
            at: at,
            bt: bt,
            ct: 0,
            tat: 0,
            wt: 0,
        });
    }
    sort_arrival(&mut process_vector_table);
    print_sjf(&mut process_vector_table);
    completion_time(&mut process_vector_table);
    print_sjf(&mut process_vector_table);

}