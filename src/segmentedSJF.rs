// use std::{cmp::min, io};

// const N:usize = 5;
// const SH:u32=10000;
// static   INT_MAX:u32 = u32::MAX;

// struct ProcessTable{
//     id:u32,
//     at:u32,
//     bt:u32,
//     ct:u32,
//     tat:u32,
//     wt:u32,
// }
// struct  SegmentTreeProcess{
//     pid:u32,
//     bt1:u32
// }

// fn input_int()->u32{
//     let mut line = String::new();
//     io::stdin().read_line(&mut line).expect("Failed to read input");
//     let number:u32=line.trim().parse().expect("!ENter a number");
//     return number;
// }

// fn sort(ar:&mut Vec<ProcessTable>){

//     for i in 0..N {
//         for j in 0..N-i-1{
//             if ar[j].at > ar[j+1].at{
//                 ar.swap(j, j+1);
//             }
//             else {
//                 if ar[j].at==ar[j+1].at {
//                     if ar[j].id > ar[j+1].id{
//                         ar.swap(j,j+1);
//                     }
//                 }
//             }
//         }
//     }
    
// }
// fn query<'a> (node:u32,st :u32, end:u32, lt:u32,rt:u32, range:&'a SegmentTreeProcess, tree : &'a Vec<SegmentTreeProcess>) -> &'a SegmentTreeProcess{
//     if end < lt || st > rt{
//         return range;
//     }
//     if st >= lt && end<=rt{
//         return &tree[node as usize];
//     }
//     let mid:u32 = (st + end)/2;
//     let lm =query(2*node+1,st,mid,lt,rt,&range,&tree );
//     let rm =query(2*node+2, mid+1,end,lt,rt, &range,&tree);
//     if lm.bt1 < rm.bt1{
//         return  lm;
//     }
//     return rm;

// }

// fn update(node:u32,st: u32,end:u32, ind:u32,id1:u32, b_t:u32, tree:&mut Vec<SegmentTreeProcess>){
//     if st== end{
//         tree[node as usize].pid =id1;
//         tree[node as usize].bt1 =b_t;
//         return;
//     }
//     let mid:u32 = (st+end)/2;
//     if ind<=mid {
//         update(2*node+1, st,mid, ind, id1, b_t, tree);
//     }
//     else{
//         update(2*node +2, mid+1,end, ind, id1, b_t, tree);
//     }
//     if tree[2*node as usize+1].bt1 <tree[2*node as usize+2].bt1 {
//         tree[node as usize+1].bt1 = tree[2*node as usize+2].bt1;
//         tree[node as usize+1].pid = tree[2*node as usize+2].pid;
//     }
//     else {
//         tree[node as usize+1].bt1 = tree[2*node as usize +2].bt1;
//         tree[node as usize+1].pid = tree[2*node as usize +2].pid;
//     }
// }

// fn sjf(ar:&mut Vec<ProcessTable>, mp:&mut [u32;SH as usize+1]){

//     let range= SegmentTreeProcess{
//         pid:INT_MAX,
//         bt1:INT_MAX,
//     };
//     let mut tree:Vec<SegmentTreeProcess>=Vec::with_capacity(4*SH as usize+1);
//     for _ in 0..4*SH as usize+1{
//         tree.push(SegmentTreeProcess { pid: INT_MAX, bt1: INT_MAX })
//     }


//     let mut counter=N as u32;
//     let mut upper_range:usize =0;
//     let mut current_running_time:u32 =min(INT_MAX,ar[upper_range].at);
//     while counter > 0 {
//         while upper_range < N -1{
//             // println!("{}",upper_range);
//             if ar[upper_range].at > current_running_time || upper_range >N {
//                 upper_range-=1;
//                 break;
//             }
//             upper_range+=1;

//             update(0,0,N as u32,upper_range as u32,ar[upper_range].id,ar[upper_range].bt,&mut tree);
            
//         }

//         let res = query(0,0,N as u32,0,upper_range as u32,&range, &tree);
//         if res.bt1 != INT_MAX{
//             counter-=1;
//             let index= mp[res.pid as usize] as usize;
//             current_running_time+=res.bt1;
//             ar[index].ct = current_running_time;
//             ar[index].tat = ar[index].ct -ar[index].at;
// 			ar[index].wt = ar[index].tat - ar[index].bt;
        

//             update(0,0,N as u32,index as u32, INT_MAX,INT_MAX,&mut tree);
//         }
//         else {
//             current_running_time = ar[upper_range].at;
//         }
//     }
// }






// fn execute(ar:&mut Vec<ProcessTable>){
//     let mut mp: [u32;SH as usize+1] =[0;SH as usize+1];
//     // print(ar);
//     sort(ar);
//     // print(ar);
//     for i in 0..N{
//         mp[ar[i].id as usize]=i as u32;
//     }
//     // println!("{}",ar[2].id);
//     sjf(ar, &mut mp);
// }

// fn print(ar: &mut Vec<ProcessTable>){
//     println!("processID Arrival Time Burst time  Completion time  turn around time  waiting time");
//     for i in 0..N{
//         println!("{}\t\t{}\t\t{}\t\t{}\t\t{}\t\t{}", ar[i].id, ar[i].at , ar[i].bt , ar[i].ct , ar[i].tat, ar[i].wt );
//     }
// }

// fn main() {
    
    
//     let mut _tree=SegmentTreeProcess{
//         pid:INT_MAX,
//         bt1:INT_MAX,
//     };
    
    
//     let mut processess:Vec<ProcessTable>=Vec::with_capacity(N);
//     // processess.push(ProcessTable { id: 0, at: 0, bt: 0, ct: 0, tat: 0, wt: 0 });

//      for _ in 0.. N{
//         let id=input_int();
//         let at = input_int();
//         let bt =input_int();
//         processess.push( ProcessTable{id: id,at:at, bt:bt,ct:0,tat :0,wt :0});
//      }

//     execute(&mut processess);
//     print(&mut processess);
// }

// 1
// 1
// 7
// 2
// 2
// 5
// 3
// 3
// 1
// 4
// 4
// 2
// 5
// 5
// 8
