use std::fs::File;

use affects_calculator::CalculatesAffects;
use affects_common::Affects;

fn main() {
    let affects: Affects = serde_json::from_reader(File::open("affects.json").unwrap()).unwrap();
    for arg in std::env::args().skip(1) {
        println!("{arg}: {:#?}", affects.calculate_affected(&arg));
    }
}
