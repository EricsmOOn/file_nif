use atoms::ok;
use rustler::{Atom, Binary};
use std::fs::File;
use std::io::Write;

mod atoms {
    rustler::atoms! {
        // Common Atoms
        ok,
        error,
    }
}

#[rustler::nif]
fn out(path: String, content: Binary) -> Atom {
    let mut f = File::create(path).unwrap();
    f.write_all(&*content).unwrap();
    ok()
}

rustler::init!("file_nif", [out]);
