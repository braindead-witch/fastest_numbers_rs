use fastest_numbers::{syllables::Dictionary, math::optimize::optimize};

fn main() {
    let dictionary = Dictionary::from_file("en-gb.json");
    let optimized = optimize(1000, &dictionary);
    println!("{:#?}", optimized);
}

