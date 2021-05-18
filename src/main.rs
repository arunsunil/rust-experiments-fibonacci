use std::env;

mod fibonacci;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./rust-experiments-fibonacci [integer > 0]");
        return;
    }

    match args[1].parse::<u32>() {
      Ok(limit) => fibonacci::fibonacci_loop(limit + 1),
      Err(error) => panic!("ERROR: Unable to convert argument to integer: {}. \
                                        \n\tUsage: ./rust-experiments-fibonacci [integer > 0]\n", error),
    };
}
