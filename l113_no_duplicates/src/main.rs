use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io, mem,
};

/// It can be tested by calling:
/// ```bash
/// cat Cargo.toml | cargo run
/// ```
fn main() {
    uniq();
}

pub fn uniq() {
    let mut last_hash = 0;
    let mut buf = String::new();

    while io::stdin().read_line(&mut buf).is_ok_and(|n| n > 0) {
        // need to create new instance to reset hasher
        let mut state = DefaultHasher::new();
        // prepare new hash value
        buf.hash(&mut state);
        // take out prepared hash
        let cur_hash = state.finish();
        // compare to last one
        if cur_hash != mem::replace(&mut last_hash, cur_hash) {
            // print if unique
            println!("{:?}", buf.trim_end());
        }
        // reset buffer
        buf.clear();
    }
}
