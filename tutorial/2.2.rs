fn main() {
    let mut p = ~[1, 2, 3];
    incrementMut(p);
    for &x in p.iter() {
        print!("{:d} ", x);
    }
    print("\n");
}

// Increment each value in `vec` by one.
fn incrementMut(vec: &mut [int]) {
    for i in range(0, vec.len()) {
        vec[i] += 1;
    }
}
