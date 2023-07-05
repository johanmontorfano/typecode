use super::tokenizer::TokenSet;
use super::reusability::ReusableDeclarations;

// Generates Rust code from TypeCode Tokens.
#[cfg(feature = "rust-gen")]
pub trait RustGen {
    fn produce_rs_build_in_single_file(
        source: Vec<TokenSet>, 
        reusability: ReusableDeclarations, 
        output_path: String) 
        -> Result<(), String>;
    fn generate_keyword_from_token_type(token: &TokenSet) -> String;
    //  Builds a type declaration, only works with inner tokens of 
    //  structs/enums.
    fn build_type_declaration(
        token: &TokenSet, reusability: &ReusableDeclarations) -> String;
}

// Generates Go code from TypeCode Tokens.
#[cfg(feature = "go-gen")]
pub trait GoGen {
    fn produce_go_build_in_single_file(
        source: Vec<TokenSet>, output_path: String) 
        -> Result<(), String>;
    fn generate_keyword_from_token_type(token: &TokenSet) -> String;
    // Builds a type declaration, only works with inner tokens of 
    // structs/enums.
    fn build_type_declaration(token: &TokenSet) -> String;
}

// Generates TypeScript code from TypeCode Tokens.
#[cfg(feature = "ts-gen")]
pub trait TSGen {
    fn produce_ts_build_in_single_file(
        source: Vec<TokenSet>,
        reusability: ReusableDeclarations,
        output_path: String)
        -> Result<(), String>;
    fn generate_keyword_from_token_type(token: &TokenSet) -> String;
    // Builds a type declaration, only works with inner tokens of
    // structs/enums.
    fn build_type_declaration(
        token: &TokenSet, reusability: &ReusableDeclarations) -> String;
}

// Generates Rust reusability statements.
#[cfg(feature = "rust-gen")]
pub trait RustReusability {
    fn produce_reusable_statement_from_struct_or_enum_token(&self) -> String;
}

// Generates TypeScript reusability statements
#[cfg(feature = "ts-gen")]
pub trait TSReusability {
    fn produce_reusable_statement_from_struct_or_enum_token(&self) -> String;
}
