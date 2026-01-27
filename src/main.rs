use fastest_numbers::{
    language::{
        Dictionary,
        counter::{EnglishRules, LanguageRuleset},
    },
    math::{export::ExportType, optimize::optimize, stats::Statistics},
};

fn main() {
    let dictionary = Dictionary::from_file("languages/en-gb.json");
    let ruleset = LanguageRuleset::English(EnglishRules);
    let optimized = optimize(100000, &dictionary, &ruleset);
    let _ = optimized.export(ExportType::Json, "results/results_en-gb.json");

    let stats = Statistics::from_optimization_result(optimized);
    println!("Statistics: {}", stats);
}
