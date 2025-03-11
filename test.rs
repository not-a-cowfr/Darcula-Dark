#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct Struct {
    value: u64,
}

const VALUE: &str = "test!";

// TODO: actually do the hundereds of todo comments in your code
fn main() {
    println!("{}\n", &VALUE);

    let num: Struct = Struct {
        value: 10,
    };

    function(num);
}

fn function(arg: Struct) -> bool {
    println!("{:#?}", arg);

    return true;
}

fn predict
