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
    print_sl(v.as_slice());
    sort(v.as_mut_slice());
    print_sl(v.as_slice());
}

fn swap(a: usize, b: usize, sl: &mut [usize]){
    let buff = sl[a];
    sl[a] = sl[b];
    sl[b] = buff;
}

fn print_sl(v: & [usize]){
    for n in v{
        print!("{}, ", n)
    }
    println!();
}

fn sort(sl: &mut [usize]){
    let sl_len = sl.len();
    if sl_len < 2 {return}

    let mut ii = 0 as usize;
    let mut is = 0 as usize;
    let piv = sl[sl_len-1];

    let mut i = 0 as usize;
    while i<sl_len{
        ii += 1;
        if sl[ii-1] <= piv{
            is += 1;
            if is != ii{
                swap(ii-1, is-1, sl)
            }
        }
        i += 1;
    }
    print_sl(sl);

    let sl_r = &mut sl[ii+1..];
    sort(&mut sl[..is-1]);
    sort(sl_r);
}