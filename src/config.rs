use std::fmt::Display;

use serde_derive::Deserialize;

use crate::{utils::file::try_read_files_from_dir_to_bytes, debug, engine::{tokenizer::TokenSet, reusability::ReusableDeclarations, generator::{TSGen, GoGen, RustGen}}};

/// Configuration extracted from command-line arguments or made up from data 
/// contained insude `TranspilerExternalConfig`.
pub struct CommandLineInstructions {
    pub transpile_to_lang: String,
    pub transpile_dir_path: String,
    pub transpile_to_output: String,
    pub go_module_name: Option<String>
}

/// Configuration of the transpiler and it's generators from a tc.conf.yaml
/// file.
#[derive(Deserialize)]
pub struct TranspilerExternalConfig {
    pub ts: Option<ClassicGeneratorConfig>,
    pub rs: Option<ClassicGeneratorConfig>,
    pub go: Option<GoGeneratorConfig>
}

/// Classic configuration of a generator.
#[derive(Deserialize, Clone)]
pub struct ClassicGeneratorConfig {
    pub output_file: String    
}

/// Go custom generator configuration.
#[derive(Deserialize, Clone)]
pub struct GoGeneratorConfig {
    pub output_file: String,
    pub module_name: String
}

impl CommandLineInstructions {
    /// Transpiles content from command-line instructions.
    pub fn transpile(&self) {
        let files = try_read_files_from_dir_to_bytes(self.transpile_dir_path.clone())
            .expect("Failed to read files from the provided directory.");

        // Transforms the `files` variable into a set of lines to process. 
        // Comments line are skipped here.
        // Transforms the files lines into a set of tokens.
        let tokenized_lines = TokenSet::apply_hierarchy_rules(files.iter()
            .map(|f| String::from_utf8_lossy(f))
            .collect::<String>()
            .trim()
            .split("\n")
            .filter(|l| !l.starts_with(":"))
            .map(|e| TokenSet::token_set_from_string(e.clone().to_string()))
            .filter(|i| i.is_some())
            .map(|f| f.unwrap())
            .collect::<Vec<TokenSet>>());
        let reusability_data = ReusableDeclarations::from_token_sets_vec(
            tokenized_lines.clone());


        debug!("File(s) content: {:?}", files);

        let result = match self.transpile_to_lang.as_str() {
            #[cfg(feature = "ts-gen")]
            "ts" => {
                <TokenSet as TSGen>::produce_ts_build_in_single_file(
                    tokenized_lines.clone(), 
                    reusability_data, 
                    self.transpile_to_output.clone())
            },
            #[cfg(feature = "go-gen")]
            "go" => {
                <TokenSet as GoGen>::produce_go_build_in_single_file(
                    tokenized_lines.clone(), 
                    reusability_data, 
                    self.transpile_to_output.clone(), 
                    self.go_module_name.clone().unwrap())
            },
            #[cfg(feature = "rust-gen")]
            "rs" => {
                <TokenSet as RustGen>::produce_rs_build_in_single_file(
                    tokenized_lines.clone(), 
                    reusability_data, 
                    self.transpile_to_output.clone())
            },
            _ => { panic!("Unknown generator: {}.", self.transpile_to_lang) }
        };

        if result.is_err() {
            println!("Producing an output failed: {}", result.err().unwrap());
        } else {
            println!("Successfully produced an output at {} in {}",
                     self.transpile_to_output,
                     self.transpile_to_lang);
        }
    }
}

impl TranspilerExternalConfig {
    pub fn make_command_line_instruction<T: Display>(
        &self, for_lang: T, dir: T) -> Option<CommandLineInstructions> {
        match for_lang.to_string().as_str() {
            "rs" => { 
                if self.rs.is_none() { return None; }
                let transpile_to_output = format!("{}/{}", 
                        dir.to_string(),
                        self.rs.clone().unwrap().output_file);

 
                Some(CommandLineInstructions {
                    transpile_to_lang: "rs".into(),
                    transpile_to_output,
                    transpile_dir_path: dir.to_string(),
                    go_module_name: None
                })
            },
            "ts" => {
                if self.ts.is_none() { return None; }
                let transpile_to_output = format!("{}/{}", 
                        dir.to_string(),
                        self.ts.clone().unwrap().output_file);

                Some(CommandLineInstructions {
                    transpile_to_lang: "ts".into(),
                    transpile_to_output,
                    transpile_dir_path: dir.to_string(),
                    go_module_name: None
                })
            },
            "go" => {
                if self.ts.is_none() { return None; }
                let transpile_to_output = format!("{}/{}", 
                        dir.to_string(),
                        self.go.clone().unwrap().output_file);

                Some(CommandLineInstructions {
                    transpile_to_lang: "go".into(),
                    transpile_to_output,
                    transpile_dir_path: dir.to_string(),
                    go_module_name: Some(self.go.clone().unwrap().module_name)
                })
            },
            _ => { panic!("Unknown generator in config: {}",
                            for_lang.to_string()); }
        }
    }
}
