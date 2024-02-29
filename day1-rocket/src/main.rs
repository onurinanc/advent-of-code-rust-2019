use std::fs;

fn get_file() -> Vec<usize>{
    let file_path = String::from("./module.txt");
    let input = fs::read_to_string(file_path).expect("msg");
    let input_parsed = input.split_whitespace()
                                                 .map(|x| x.parse::<usize>().unwrap())
                                                 .collect::<Vec<usize>>();
    input_parsed
}

fn calculate_module_fuel(mass: &usize) -> usize {
    let round_down = mass / 3; // using usize automatically rounds down
    let res = round_down - 2;
    res
}

fn calculate_total_fuel() -> usize {
    let file = get_file();
    file.iter().map(|x| calculate_module_fuel(x)).sum()
}

fn main() {
    println!("Total fuel need is {:?}", calculate_total_fuel());
}







/*use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}*/