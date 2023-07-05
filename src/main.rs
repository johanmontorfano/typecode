use utils::clargs;
use utils::file::try_read_files_from_dir_to_bytes;
use engine::tokenizer::TokenSet;

use crate::engine::{generator::{RustGen, GoGen, TSGen}, reusability::ReusableDeclarations};

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
        .expect("Missing input directory path").clone();
    let (_, output) = 
        clargs::argument_and_param_from_args_string_vec(&cli_args, "-o".into())
        .expect("Missing output file destination: -o [filename]");
    let (_, lang) = 
        clargs::argument_and_param_from_args_string_vec(&cli_args, "-l".into())
        .expect("Missing language output: -l [language]");


    println!("Transpiling typecode from {} into {} using the {} generator.", 
             dir_path, output, lang.to_uppercase());

    // Read all files from the directory.
    let files = try_read_files_from_dir_to_bytes(dir_path)
        .expect("Failed to read files from the provided directory");
    
    files.iter().for_each(|f| {
        let string_file = String::from_utf8(f.to_vec()).unwrap().trim()
            .split("\n").map(|line| line.to_string())
            .collect::<Vec<String>>();

        debug!("Processing file: \n{}", string_file.join("\n"));

        let parsed_lines = string_file.iter()
            .map(|line| TokenSet::token_set_from_string(line.clone()))
            .filter(|item| item.is_some())
            .map(|filtered| filtered.unwrap())
            .collect::<Vec<TokenSet>>();

        let hierachized_lines = TokenSet::apply_hierarchy_rules(parsed_lines);
        let flatten_reusability = 
            ReusableDeclarations::from_token_sets_vec(
                hierachized_lines.clone());

        if let Err(reason) = match lang.to_lowercase().as_str() {
            #[cfg(feature = "ts-gen")]
            "ts" | "typescript" => {
                <TokenSet as TSGen>::produce_ts_build_in_single_file(
                    hierachized_lines.clone(), 
                    flatten_reusability,
                    output.clone())
            },
            #[cfg(feature = "go-gen")]
            "go" | "golang" => {
                <TokenSet as GoGen>::produce_go_build_in_single_file(
                    hierachized_lines.clone(), 
                    flatten_reusability,
                    output.clone())
            },
            #[cfg(feature = "rust-gen")]
            "rs" | "rust" => {
                <TokenSet as RustGen>::produce_rs_build_in_single_file(
                    hierachized_lines.clone(), 
                    flatten_reusability, 
                    output.clone())
            },
            _ => { panic!("Unknwon generator: {}.", lang.to_uppercase()) }
        } {
            println!("Failed to produce output: {}", reason);
        } else {
            println!("Successfully produced an output at {}", output.clone());
        }
    });
}

