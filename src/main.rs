use fastest_numbers::{
    math::{export::ExportType, optimize::optimize, stats::Statistics},
    syllables::Dictionary,
};

fn main() {
    let dictionary = Dictionary::from_file("languages/en-gb.json");
    let optimized = optimize(100000, &dictionary);
    let _ = optimized.export(ExportType::Json, "results/results_en-gb.json");

    let stats = Statistics::from_optimization_result(optimized);
    println!("Statistics: {}", stats);
}
