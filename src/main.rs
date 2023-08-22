use utils::clargs;
use crate::{
    utils::conf_file::{
        try_detect_conf_file_within_provided_directory, 
        read_configuration_from_to_config_struct}, 
    config::CommandLineInstructions};

mod config;
mod engine;
mod utils;
mod macros;

static mut DEBUG_ENABLED: bool = false;

fn main() {
    let cli_args = clargs::cli_args_to_string_vec();

    // Environment settings
    unsafe {
        DEBUG_ENABLED = clargs::is_argument_present_on_args_string_vec(
            &cli_args, "--debug".into());
    }

    // Command-line variables
    let dir_path = cli_args.get(1)
        .expect("Missing input directory path");

    // Buffer that contains every transpilation to do on the current session.
    // It's only useful with configuration files that requires multiple 
    // transpilations per session.
    let mut tpl_instrs: Vec<CommandLineInstructions> = vec![];

    // If a `tc.conf.file` is found, we'll use the `TranspilerExternalConfig`
    // struct to handle transpilation requests, otherwise we'll manually parse
    // it into `CommandLineInstructions`.
    if try_detect_conf_file_within_provided_directory(dir_path.clone()) {
        let conf_file_content = read_configuration_from_to_config_struct(
            dir_path.clone());
        if conf_file_content.ts.is_some() {
            tpl_instrs.push(conf_file_content.make_command_line_instruction(
                    "ts", &dir_path).unwrap()); 
        }
        
        if conf_file_content.rs.is_some() {
            tpl_instrs.push(conf_file_content.make_command_line_instruction(
                    "rs", &dir_path).unwrap());
        }

        if conf_file_content.go.is_some() {
            tpl_instrs.push(conf_file_content.make_command_line_instruction(
                    "go", &dir_path).unwrap());
        }
        
    } else {
        let (_, output) = clargs::argument_and_param_from_args_string_vec(
            &cli_args, "-o".into())
            .expect("Missing output file destination: -o [filename]");
        let (_, lang) = clargs::argument_and_param_from_args_string_vec(
            &cli_args, "-l".into())
            .expect("Missing language output: ");
        let mut go_module_name = None;

        if lang == "go".to_string() {
            go_module_name = Some(
                clargs::argument_and_param_from_args_string_vec(
                    &cli_args, 
                    "--go-package-name".into())
               .expect("Missing `--go-package-name [name]` argument.")
               .1); 
        }

        tpl_instrs.push(CommandLineInstructions { 
            transpile_to_lang: lang, 
            transpile_dir_path: dir_path.clone(), 
            transpile_to_output: format!("{}/{}", dir_path.clone(), output), 
            go_module_name });
    }

    // Process every transpilation instruction.
    tpl_instrs.iter().for_each(|tpl| {
        println!("Processing files from {} with the {} generator, and outputs to {}.",
                 dir_path, tpl.transpile_to_lang, tpl.transpile_to_output);
        tpl.transpile();
    });


}

