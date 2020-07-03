// Prints numbers from 1 to 100. If number is mulptiple of 3 it prints "fizz",
// if number is multiple of 5 it prints "buzz", if number is multiple of 15
// it prints fizzbuzz

fn main() {
    for n in 1..100 {
        let mut to_print = String::new();
        let mut changed: bool = false;

        match n % 3 {
            0 => {
                to_print.push_str("fizz");
                changed = true;
            }
            _ => (),
        }

        match n % 5 {
            0 => {
                to_print.push_str("buzz");
                changed = true;
            }
            _ => (),
        }

        if !changed {
            to_print.push_str(&format!("{}", n));
        }

        println!("{}", to_print);
    }
}
