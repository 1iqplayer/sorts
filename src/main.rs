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
    print_v(&v);
    let len = v.len();
    sort(&mut v, 0, len-1);
    print_v(&v);
}

fn print_v(v: & Vec<usize>){
    for n in v{
        print!("{}, ", n)
    }
    println!();
}

fn sort(v: &mut Vec<usize>,start: usize, end: usize){
    let v_len = end - start;

    let mut ii = 0 as usize;
    let mut is = 0 as usize;
    let piv = v[v_len];

    let mut i = 0 as usize;
    while i<= v_len{
        ii += 1;
        if v[ii-1] <= piv{
            is += 1;
            if is != ii{
                v.swap(ii-1, is-1);//ap(ii-1, is-1, v)
            }
        }
        i += 1;
    }
    print_v(&v);
    let l_start = start;
    let l_end = is-2;
    let r_start = is;
    let r_end = end;
    if l_start != l_end {sort(v, start, is-2);}
    if r_start != end {sort(v, r_start, r_end);}
}