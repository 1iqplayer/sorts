use std::io;
//use std::vec;
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
    sort.sort_bucket();

    
}

impl sortx {
    fn sort_quick(&self) -> bool{
        let mut cpy = self.arr.to_vec();
        self._quicksort(&mut cpy, 0, self.arr_size-1);

        self.test(&cpy)
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

    fn sort_bubble(&self) -> bool{
        let mut i = self.arr_size-1;
        let mut cpy = self.arr.to_vec();

        if self.dec{
            while i > 0{
                let mut swap = 0;
                let mut seq = true;
                for j in 0..=i{
                    if cpy[j] < cpy[swap]{
                        swap = j;
                    }else{seq = false}
                }
                if seq == true {break}
                cpy.swap(swap, i);
                i -= 1;
            }
        }else{
            while i > 0{
                let mut swap = 0;
                let mut seq = true;
                for j in 0..=i{
                    if cpy[j] > cpy[swap]{
                        swap = j;
                    }else{seq = false}
                }
                if seq == true {break}
                cpy.swap(swap, i);
                i -= 1;
            }
        }
        
        self.test(&cpy)
    }

    fn sort_bubblev2(&self)->bool{
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
        
        self.test(&cpy)
    }

    fn sort_insert(&self)->bool{
        let mut cpy = self.arr.to_vec();

        if self.dec{
            for i in 1..self.arr_size{
                let mut j = i;
                while j>0 && cpy[j-1]<cpy[j]{
                    cpy.swap(j-1, j);
                    j -= 1;
                }
            }
        }else{
            for i in 1..self.arr_size{
                let mut j = i;
                while j>0 && cpy[j-1]>cpy[j]{
                    cpy.swap(j-1, j);
                    j -= 1;
                }
            }
        }
        self.test(&cpy)
    }

    fn sort_bucket(&self)->bool{
        let mut cpy = self.arr.to_vec();
        let n = 10;
        let range = self.arr_range / n;

        let mut v: Vec<Vec<usize>>   = vec![vec![]; n+1];

        // Insert elements in buckets
        for i in 0..self.arr_size{
            let bi = cpy[i]/range;
            v[bi].push(cpy[i]);
        }

        // Sort buckets

        for i in 0..n+1{
            if v[i].len()>1 {sortx::_insert(&mut v[i]);}
        }

        let mut count = 0;
        for i in 0..n+1{
            if v[i].len()>0 {
                for j in 0..v[i].len(){
                    cpy[count] = v[i][j];
                    count += 1;
                }
            }
        }
        println!("{:#?}", cpy);
        self.test(&cpy)
    }

    fn _insert<T: Ord>(v: &mut Vec<T>){
        for i in 1..v.len(){
            let mut j = i;
            while j>0 && v[j-1]>v[j]{
                v.swap(j-1, j);
                j -= 1;
            }
        }
    }
    fn test(&self, v: & Vec<usize>)->bool{
        if self.dec{
            for i in 0..self.arr_size-1{
                if v[i] < v[i+1] && v[i] != v[i+1] {
                    println!("!! array sorted incorrectly (decreasing) !!");
                    return false
                }
            }
        }else{
            for i in 0..self.arr_size-1{
                if v[i] > v[i+1] && v[i] != v[i+1] { 
                    println!("!! array sorted incorrectly (increasing) [{}] !!", v[i]);
                    return false
                }
            }
        }
        println!("--array sorted correctly--");
        true
    }

    fn arr_gen(&mut self){
        let mut gen = rand::thread_rng();
        self.arr.clear();
        for _ in 0..self.arr_size{
            self.arr.push(gen.gen_range(0..=self.arr_range));
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

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::sortx;

    #[test]
    fn test_bucket(){
        let mut n = 10;

        let mut sort = sortx{
            arr_size: 10,
            arr_range: 20,
            arr: Vec::new(),
            dec: false
        };

        while n>0{
            println!("{n}");
            n -= 1;
            sort.arr_size = rand::thread_rng().gen_range(1000..10000);
            sort.arr_range = rand::thread_rng().gen_range(0..60000);
            sort.arr_gen();
            assert_eq!(sort.sort_bucket(), true);
        }
    }
}