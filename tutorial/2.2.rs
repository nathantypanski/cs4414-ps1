fn main() {
    let mut p = ~[1, 2, 3];
    incrementMut(p);
    for &x in q.iter() {
        print!("{:d} ", x);
    }
    print("\n");
}

fn incrementMut(vec: &[int]) -> &[int] {
    for &x in vec.iter() {
        x = x + 1;
    }
}
