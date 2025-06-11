mod filter;
pub mod parser;
pub mod processor;
pub mod utils;

pub fn filter_data(data: String) -> Vec<f64> {
    let parsed = parser::Parser::run(&data);
    parsed.display();

    let processor = processor::Processor::run(parsed.parsed_data.unwrap());
    processor.display();

    let filtered_data = processor.filtered_data.unwrap_or_default();
    filtered_data
}
