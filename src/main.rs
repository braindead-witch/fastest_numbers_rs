use fastest_numbers::{
    language::{
        Dictionary,
        counter::{DutchRules, LanguageRuleset},
    },
    math::{export::ExportType, optimize::optimize, stats::Statistics},
};

fn main() {
    let dictionary = Dictionary::from_file("languages/nl-nl.json");
    let ruleset = LanguageRuleset::Dutch(DutchRules);
    let optimized = optimize(100000, &dictionary, &ruleset);
    let _ = optimized.export(ExportType::Json, "results/results_nl-nl.json");

    let stats = Statistics::from_optimization_result(optimized);
    println!("Statistics: {}", stats);
}
