use fastest_numbers::{math::optimize::optimize, syllables::Dictionary};

fn main() {
    let dictionary = Dictionary::from_file("en-gb.json");
    let optimized = optimize(100000, &dictionary);
    println!("{:#?}", optimized.inner);
}
