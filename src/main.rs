use rusty_pedometer::filter_data;

fn main() {
    // Load data from file passed in as argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let data = std::fs::read_to_string(filename).expect("Unable to read file");

    let filtered_data = filter_data(data);
    println!("Filtered data: {:#?}", filtered_data);
}
