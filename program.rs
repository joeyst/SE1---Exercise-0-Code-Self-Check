use std::collections::HashMap;
use std::io::{stdin,stdout,Write};
use std::num::ParseFloatError;

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

fn parse_arguments(args: Vec<&str>) -> Vec<Result<f64, ParseFloatError>> {
  args[1..].into_iter().map(|arg| arg.parse::<f64>()).collect()
}

fn all_arguments_are_ok(args: Vec<Result<f64, ParseFloatError>>) -> bool {
  args.clone().into_iter().all(|arg| match arg { Err(x) => false, Ok(x) => true})
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

    // if only one argument, make sure it's quit. 
    // If it's quit, exit program, else print options message and restart 'program loop. 
    if number_of_args == 1 {
      match args[0] {
        "quit" => break 'program,
        _ => print_options()
      }
    } 
    // if there are three arguments, make sure that the first argument is in the operators map. 
    // if it is, perform the operation on it, print the result, and restart loop. 
    // if it's not, print options message and restart 'program loop. 
    // I wrote this so that more than just 2 arguments can be used-- the number_of_args == 3 would just need to be changed. 
    // if the user input can't be coerced into a float, print the options message and restart the 'program loop. 
    else if number_of_args == 3 {
      // check if operator in operators map
      if ops_map.contains_key(args[0].clone()) {
        // try to convert user input strings into floats. If can't coerce, will turn into Err(x), otherwise, will be Ok(x)
        let parsed_args = parse_arguments(args.clone());
        // match each arg in parsed_args to be false if it's an error and true if it's not, and make sure all are not errors. 
        if all_arguments_are_ok(parsed_args.clone()) {
          // calculate the value based on the operation and the arguments
          let value = ops_map[args[0]](args[1..].into_iter().map(|arg| arg.parse::<f64>().unwrap()).collect::<Vec<f64>>());
          // print the value, restart 'program loop. 
          println!("{:?}", value);
        } 
        else {
          // if there are any errors, print usage options, and restart 'program loop. 
          print_options();
        }

      // if the operators map doesn't contain the first word the user entered, print usage options, and restart 'program loop.   
      } else {
        print_options();
      }
    } 
    // the number of arguments isn't 1 or 3, so print the usage options, and restart the 'program loop. 
    else {
      print_options();
    }
  }
  
}

// add, sub, ... => take 1 and fold rest. for now, ask user again if total args != 3
// quit => exit loop
// _ => say error and ask again