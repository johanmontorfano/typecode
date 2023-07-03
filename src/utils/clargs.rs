use std::env;

// Read all the command line args and returns them as a Vec<String>.
pub fn cli_args_to_string_vec() -> Vec<String> {
    env::args().collect::<_>()
}

// Read command line arguments and returns the position of an argument.
pub fn argument_position_in_args_string_vec(
    args_vec: &Vec<String>, arg_title: String) -> Option<usize> {
    args_vec.iter().position(|v| v == &arg_title)
}

// Get an argument title and it's associated parameter such as 
// `-[argument title] [parameter]`.
pub fn argument_and_param_from_args_string_vec(
    args_vec: &Vec<String>, arg_title: String) -> Option<(String, String)> {
    match argument_position_in_args_string_vec(args_vec, arg_title.clone()) {
        Some(pos) => { Some((arg_title, 
                             args_vec.get(pos + 1).unwrap().clone())) }
        None => { None }
    }
}

// Check for the presence of an argument.
pub fn is_argument_present_on_args_string_vec(
    args_vec: &Vec<String>, arg: String) -> bool {
    argument_position_in_args_string_vec(args_vec, arg).is_some()
}
