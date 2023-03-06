use std::io;
use std::vec;
use rand::Rng;
use std::time;

fn main() {
    
    let n = parse_int("Enter no elements: ");
    let max = parse_int("Enter max size of element: ");
    let inc = parse_int("Sort increasing? yes-1 no-0") != 0;
    
    let mut sort = sortx{
        arr_size: n,
        arr_range: max,
        arr: Vec::new(),
        dec: !inc
    };

    sort.arr_gen();
    //sort.sort_quick();
    sort.sort_bubble();
    //sort.sort_bubblev2();
}

impl sortx {
    fn sort_quick(&self){
        let mut cpy = self.arr.to_vec();
        self._quicksort(&mut cpy, 0, self.arr_size-1);

        if self.dec  {self.dec_test(&mut cpy);}
        else    {self.inc_test(&mut cpy);}
        println!("{:?}", cpy);
    }

    fn _quicksort(&self, v: &mut Vec<usize>, low: usize, high: usize){
        let p = self.partition(v, low, high);
        if low != p {self._quicksort(v, low, p-1);}
        if p != high {self._quicksort(v, p+1, high);}
    }

    fn partition(&self, v: &mut Vec<usize>, low: usize, high: usize) -> usize{
        let mut ii = low;
        let mut is = low;
        if self.dec{

            for _i in low..=high{
                ii += 1;
                if v[ii-1] >= v[high]{
                    is += 1;
                    if ii != is{
                        v.swap(ii-1, is-1)
                    }
                }
            }
        }else{
            for _i in low..=high{
                ii += 1;
                if v[ii-1] <= v[high]{
                    is += 1;
                    if ii != is{
                        v.swap(ii-1, is-1)
                    }
                }
            }
        }
        is-1
    }

    fn sort_bubble(&self){
        let mut i = self.arr_size-1;
        let mut cpy = self.arr.to_vec();

        if self.dec{
            while i > 1{
                let mut swap = 0;
                let mut seq = true;
                for j in 1..=i{
                    if cpy[j] < cpy[swap]{
                        swap = j;
                    }else{seq = false}
                }
                if seq == true {break}
                cpy.swap(swap, i);
                i -= 1;
            }
        }else{
            while i > 1{
                let mut swap = 0;
                let mut seq = true;
                for j in 1..=i{
                    if cpy[j] > cpy[swap]{
                        swap = j;
                    }else{seq = false}
                }
                if seq == true {break}
                cpy.swap(swap, i);
                i -= 1;
            }
        }
        
        println!("{:?}", cpy);
        if self.dec  {self.dec_test(&mut cpy);}
        else    {self.inc_test(&mut cpy);}
    }

    fn sort_bubblev2(&self){
        let n = self.arr_size;
        let mut cpy = self.arr.to_vec();

        if self.dec{
            'outer: for i in 0..n-1{
                let mut seq = true;
                for j in 0..n-i-1{
                    if cpy[j] < cpy[j+1] {
                        cpy.swap(j, j+1);
                        seq = false;
                    }
                }
                if seq {break 'outer}
            }
        }else{
            'outer: for i in 0..n-1{
                let mut seq = true;
                for j in 0..n-i-1{
                    if cpy[j]>cpy[j+1] {
                        cpy.swap(j, j+1);
                        seq = false;
                    }
                }
                if seq {break 'outer}
            }
        }
        
        println!("{:?}", cpy);
        if self.dec  {self.dec_test(&cpy);}
        else    {self.inc_test(&cpy);}
    }

    fn dec_test(&self, v: & Vec<usize>)->bool{
        for i in 0..self.arr_size-1{
            if v[i] < v[i+1] && v[i] != v[i+1] {
                println!("!! array sorted incorrectly (decreasing) !!");
                return false
            }
        }
        println!("array sorted correctly (decreasing)");
        true
    }

    fn inc_test(&self, v: & Vec<usize>)->bool{
        for i in 0..self.arr_size-1{
            if v[i] > v[i+1] && v[i] != v[i+1] { 
                println!("!! array sorted incorrectly (increasing) [{}] !!", v[i]);
                return false
            }
        }
        println!("array sorted correctly (increasing)");
        true
    }

    fn arr_gen(&mut self){
        let mut gen = rand::thread_rng();
        for _ in 0..self.arr_size{
            &self.arr.push(gen.gen_range(0..=self.arr_range));
        }
    }
}
struct sortx{
    arr_size: usize,
    arr_range: usize,
    arr: Vec<usize>,
    dec: bool
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
