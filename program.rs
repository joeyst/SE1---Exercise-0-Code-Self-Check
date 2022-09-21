use std::collections::HashMap;
use std::io::{stdin,stdout,Write};

fn add(values: Vec<f64>) -> f64 {
  values.into_iter().reduce(|a, b| a + b).unwrap()
}

fn sub(values: Vec<f64>) -> f64 {
  values.into_iter().reduce(|a, b| a - b).unwrap()
}

fn mul(values: Vec<f64>) -> f64 {
  values.into_iter().reduce(|a, b| a * b).unwrap()
}

fn div(values: Vec<f64>) -> f64 {
  values.into_iter().reduce(|a, b| a / b).unwrap()
}

fn print_options() {
  println!("usage: add|sub|mul|div v0 v1\nquit");
}

fn main () {

  let mut ops_map: HashMap<&str, fn(Vec<f64>) -> f64> = HashMap::new();
  ops_map.insert("add", add);
  ops_map.insert("sub", sub);
  ops_map.insert("mul", mul);
  ops_map.insert("div", div);

  'program: loop {

    let mut user_input: String = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    (*user_input).trim();

    // code found here: 
    // https://www.reddit.com/r/rust/comments/bhl85n/tips_tricks_ways_to_remove_trailing_newlines/

    let len = user_input.trim_end_matches(&['\r', '\n'][..]).len();
    user_input.truncate(len);

    // end of taken code

    // splitting user input into dif arguments, as strings
    let args = user_input.split(" ").collect::<Vec<&str>>();
    args.clone().into_iter().for_each(|x| println!("{:?}", x));

    let number_of_args = args.len();
    println!("Number of arguments: {:?}", number_of_args);

    if number_of_args == 1 {
      match args[0] {
        "quit" => break 'program,
        _ => print_options()
      }
    } else if number_of_args == 3 {
      if ops_map.contains_key(args[0]) {
        let parsed_args = args[1..].into_iter().map(|arg| arg.parse::<f64>());
        if parsed_args.clone().all(|arg| match arg { Err(x) => false, Ok(x) => true}) {
          let value = ops_map[args[0]](args[1..].into_iter().map(|arg| arg.parse::<f64>().unwrap()).collect::<Vec<f64>>());
          println!("{:?}", value);
        } else {
          print_options();
        }
        
      } else {
        print_options();
      }
    } else {
      print_options();
    }
  }
  
}

// add, sub, ... => take 1 and fold rest. for now, ask user again if total args != 3
// quit => exit loop
// _ => say error and ask again