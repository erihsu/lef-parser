use std::fs;
use std::time::*;
fn main() {
    use lef_parser::LefData;
    let now = SystemTime::now();
    let _lef_data: LefData = fs::read_to_string("example_cell.lef")
        .unwrap()
        .parse()
        .unwrap();

    let get_tf = SystemTime::now();

    println!("Successfully. Time used: {:?}.", get_tf.duration_since(now));
}
