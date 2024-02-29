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
    let round_down = mass / 3;
    
    if round_down > 2 {
        round_down - 2
    } else {
        0
    }
}

fn calculate_full_fuel(mass: &usize) -> usize {
    let fuel = calculate_module_fuel(mass);

    if fuel > 0 {
        fuel + calculate_full_fuel(&fuel)
    } else {
        fuel
    }
}

fn calculate_total_fuel_part_1() -> usize {
    let file = get_file();
    file.iter().map(|x| calculate_module_fuel(x)).sum()
}

fn calculate_total_fuel_part_2() -> usize {
    let file = get_file();
    file.iter().map(|x| calculate_full_fuel(x)).sum()
}



fn main() {
    println!("Total fuel need is {:?}", calculate_total_fuel_part_1());
    println!("Total fuel need is {:?}", calculate_total_fuel_part_2());
}