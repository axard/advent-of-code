use md5::{Digest, Md5};

fn main() {
    let input = b"iwrupvqb";
    let mut hasher = Md5::new();

    for i in 0..u64::MAX {
        hasher.update(input);
        hasher.update(i.to_string().as_bytes());

        // let mut output: [u8; 16] = [0; 16];
        let output = hasher.finalize_reset();
        let first_five = output[0] as u32 + output[1] as u32 + (output[2] >> 4) as u32;
        if first_five == 0 {
            println!("{}", i);
            break;
        }
    }
}
