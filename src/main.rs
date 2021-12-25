use std::io;
const N: usize = 5;
struct ProcessTable {
    id: u32,
    at: u32,
    bt: u32,
    ct: u32,
    tat: u32,
    wt: u32,
}
fn sort_arrival(ar: &mut Vec<ProcessTable>) {
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
fn print(ar: &mut Vec<ProcessTable>) {
    println!("processID Arrival Time Burst time  Completion time  turn around time  waiting time");
    for i in 0..N {
        println!(
            "{}\t\t{}\t\t\t{}\t\t{}\t\t{}\t\t{}",
            ar[i].id, ar[i].at, ar[i].bt, ar[i].ct, ar[i].tat, ar[i].wt
        );
    }
}
fn input_int() -> u32 {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read input");
    let number: u32 = line.trim().parse().expect("!ENter a number");
    return number;
}
fn completion_time(vector: &mut Vec<ProcessTable>) {
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
fn main() {
    println!("The number of processes: {}", N);
    let mut process_vector_table: Vec<ProcessTable> = Vec::with_capacity(N);
    for _ in 0..N {
        println!("Enter the process ID: ");
        let id = input_int();
        println!("Enter the Arrival Time: ");
        let at = input_int();
        println!("Enter the Burst Time: ");
        let bt = input_int();
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
    print(&mut process_vector_table);
    completion_time(&mut process_vector_table);
    print(&mut process_vector_table);
}
