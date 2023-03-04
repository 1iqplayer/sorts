use std::io;
use std::vec;
use rand::Rng;

fn main() {
    let mut buff = String::new();
    io::stdin()
        .read_line(&mut buff)
        .expect("failed to read line!");
    let num: usize = buff.trim().parse().expect("input is not a number!");
    println!("generating random array of {} elements", num);

    let mut i = 0;
    let mut v: Vec<usize> = vec::Vec::new();
    while i<num{
        v.push(rand::thread_rng().gen_range(0..=20));
        i += 1;
    }   
    println!("Before: {:?}", &v);
    quicksort(v.as_mut_slice());
    println!("after:  {:?}", &v);
}
fn quicksort<T: Ord + std::fmt::Debug>(sl: &mut [T]){
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