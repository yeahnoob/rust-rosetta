// http://rosettacode.org/wiki/Hello_world/Line_printer

use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::open("/dev/lp0").unwrap();
    write!(file, "Hello World!").unwrap();
}
