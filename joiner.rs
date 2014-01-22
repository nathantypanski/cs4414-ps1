use std::os;
use std::str;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <msg.share1> <msg.share2>", args[0]); 
    } else {
        let share1 = args[1].clone();
        let share2 = args[2].clone();
        let path1 = Path::new(share1.clone());
        let path2 = Path::new(share2.clone());
        let msg1_file = File::open(&path1);
        let msg2_file = File::open(&path2);

        match (msg1_file, msg2_file) {
            (Some(mut msg1), Some(mut msg2)) => {
                let msg1_bytes: ~[u8] = msg1.read_to_end();
                let msg2_bytes: ~[u8] = msg2.read_to_end();
                let result: ~[u8] = xor(msg1_bytes, msg2_bytes);
                print!("{:?}", str::from_utf8(result));
            } ,
            (None, _ ) => fail!("Error opening message file: {:s}", share1),
            (_ ,None) => fail!("Error opening message file: {:s}", share2)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}
