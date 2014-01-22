fn main() {
   let p = ~[1, 2, 3];
    let q = increment(p);
    for &x in q.iter() {
        print!("{:d} ", x);
    }
    print("\n");
}

fn increment(vec: ~[int]) -> ~[int] {
    let mut ret = ~[];
    for &x in vec.iter() {
        ret.push(x + 1);
    }
    ret
}
