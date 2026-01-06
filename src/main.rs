use fastest_numbers::syllables::Dictionary;

fn main() {
    let dictionary = Dictionary::from_file("en-gb.json");

    println!("{:#?}", dictionary);
}

