use std::io;
use std::vec;
use rand::Rng;
use std::time;

fn main() {
    
    let n = parse_int("Enter no elements: ");
    let max = parse_int("Enter max size of element: ");
    
    print!("generating random array of {} elements... ", n);
    let mut now = time::Instant::now();
    let mut v: Vec<usize> = vec::Vec::new();
    for _ in 0..n{
        v.push(rand::thread_rng().gen_range(0..=max));
    }   
    print!("Done in {}ms", now.elapsed().as_millis());

    print!("Sorting bubble...");
    let mut fresh = v.to_vec();
    now = time::Instant::now();
    sort_bubble(fresh.as_mut_slice());
    println!(" Done in {}ms", now.elapsed().as_millis());

    print!("Sorting bubblev2...");
    let mut fresh = v.to_vec();
    now = time::Instant::now();
    sort_bubblev2(fresh.as_mut_slice());
    println!(" Done in {}ms", now.elapsed().as_millis());

    print!("Sorting quick...");
    let mut fresh = v.to_vec();
    now = time::Instant::now();
    sort_quick(fresh.as_mut_slice());
    println!(" Done in {}ms", now.elapsed().as_millis());
}
fn sort_quick<T: Ord + std::fmt::Debug>(sl: &mut [T]){
    let sl_len = sl.len();
    _quicksort(sl, 0, sl_len-1)
}

fn _quicksort<T: Ord + std::fmt::Debug>(sl: &mut [T], low: usize, high: usize){
    let p = partition(sl, low, high);
    if low != p {_quicksort(sl, low, p-1);}
    if p != high {_quicksort(sl, p+1, high);}

}

fn partition<T: Ord>(sl: &mut [T], low: usize, high: usize) -> usize{
    let mut ii = low;
    let mut is = low;
    for _i in low..=high{
        ii += 1;
        if sl[ii-1] <= sl[high]{
            is += 1;
            if ii != is{
                sl.swap(ii-1, is-1)
            }
        }
    }
    is-1
}

fn sort_bubble<T: Ord>(sl: &mut [T]){
    let mut i = sl.len()-1;
    while i > 1{
        let mut swap = 0;
        let mut seq = true;
        for j in 1..=i{
            if sl[j] > sl[swap]{
                swap = j;
            }else{seq = false}
        }
        if seq == true {break}
        sl.swap(swap, i);
        i -= 1;
    }
}

fn sort_bubblev2<T: Ord>(sl: &mut [T]){
    let n = sl.len();
    for i in 0..n-1{
        for j in 0..n-i-1{
            if sl[j]>sl[j+1] {sl.swap(j, j+1)}
        }
    }
}

fn parse_int(msg: &str)->usize{
    print!("\n{}", msg);
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut buff = String::new();
    io::stdin()
        .read_line(&mut buff)
        .expect("failed to read from input!");
    let num: usize = buff.trim().parse().expect("invalid number!");
    num
}