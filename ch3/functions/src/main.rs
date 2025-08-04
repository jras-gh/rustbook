fn main() {
    another_function(10);
    print_labeled_measurements(5, 'h');
    another_function(five());
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
