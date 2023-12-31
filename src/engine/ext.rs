use heck::{ToLowerCamelCase, ToSnakeCase};

use crate::utils::file::try_write_bytes_to_file;
use crate::{debug, warn};
use super::reusability::{ItemDeclarationDescriptor, ReusableDeclarations};
use super::tokenizer::{TokenSet, TokenType, TokenParameter};


// ITEM DECLARATION DESCRIPTOR IMPLEMENTATIONS
#[cfg(feature = "rust-gen")]
use super::generator::RustReusability;
#[cfg(feature = "rust-gen")]
impl RustReusability for ItemDeclarationDescriptor {
    fn produce_reusable_statement_from_struct_or_enum_token(&self) -> String {
        return format!("super::{}::{}", 
                       self.module_name, self.declaration_name);
    }
}

#[cfg(feature = "ts-gen")]
use super::generator::TSReusability;
#[cfg(feature = "ts-gen")]
impl TSReusability for ItemDeclarationDescriptor {
    fn produce_reusable_statement_from_struct_or_enum_token(&self) -> String {
        return format!("{}.{}", self.module_name, self.declaration_name);
    }
}


// TOKEN SET IMPLEMENTATIONS
#[cfg(feature = "rust-gen")]
use super::generator::RustGen;
#[cfg(feature = "rust-gen")]
impl RustGen for TokenSet {
    fn generate_keyword_from_token_type(token: &TokenSet) -> String {
        match token.token_type {
            TokenType::Char => { "char" }
            TokenType::Bool => { "bool" }
            TokenType::IntU8 => { "u8" }
            TokenType::IntI8 => { "i8" }
            TokenType::Module => { "mod" }
            TokenType::String => { "String" }
            TokenType::IntU16 => { "u16" }
            TokenType::IntI16 => { "i16" }
            TokenType::IntU32 => { "u32" }
            TokenType::IntI32 => { "i32" }
            TokenType::IntU64 => { "u64" }
            TokenType::IntI64 => { "i64" }
            TokenType::Structure => { "struct" }
            TokenType::Enumeration => { "enum" }
            TokenType::Custom => { 
                return token.clone().custom_token_type.unwrap(); }
        }.into()
    }

    fn build_type_declaration(
            token: &TokenSet, reusability: &ReusableDeclarations) -> String {
        let mut output_type = format!("{}", <TokenSet as RustGen>::
                                  generate_keyword_from_token_type(token));

        if token.parameters.contains(&TokenParameter::LocalType) {
            let reusable_data = reusability.
                    find_declaration_descriptor_with_declaration_name( 
                        token.custom_token_type.clone().unwrap());

            debug!("Found a token with the `local` parameter.");

            if reusable_data.is_some() {
                output_type = <ItemDeclarationDescriptor as RustReusability>
                    ::produce_reusable_statement_from_struct_or_enum_token(
                        reusable_data.unwrap());
            } 
        }

        if token.parameters.contains(&TokenParameter::Optional) {
            output_type = format!("Option<{}>", output_type);
        }

        if token.parameters.contains(&TokenParameter::Floated) {
            match token.token_type {
                TokenType::IntU32 | TokenType::IntI32| TokenType::IntU64 
                    | TokenType::IntI64 => {
                        output_type = output_type.replace("u", "f")
                            .replace("i", "f");
                    }
                _ => { 
                    warn!("Rust: Floated applies only to [u|i][32|64] types.") 
                } 
            }
        }

       if token.parameters.contains(&TokenParameter::Pointer) ||
           token.parameters.contains(&TokenParameter::Reference) {
            warn!("Rust: pointer and ref does the same thing.");
            
            output_type = format!("&{}", output_type);
        }

       if token.parameters.contains(&TokenParameter::Vector) {
            output_type = format!("Vec<{}>", output_type);
       }

       return format!("pub {}: {}", 
                      token.token_name.to_snake_case(), output_type)
    }

    fn produce_rs_build_in_single_file(
            source: Vec<TokenSet>,
            reusability: ReusableDeclarations,
            output_path: String) -> Result<(), String> {
        // Content is generated line by line and is put here before being 
        // joined at save time.
        let mut content_lines: Vec<String> = vec![];

        for root_item in source {
            // When generating Rust code, a root_item always has to be a
            // module.
            if root_item.token_type != TokenType::Module { return 
                Err("Modules have to be declared before anything.".into()) }

            content_lines.push(format!("pub mod {} {{", root_item.token_name));
            
            // Loops through the Module's childs.
            for secondary_item /* Such as struct or enum. */ in 
                root_item.childs {
                if secondary_item.token_type != TokenType::Structure &&
                    secondary_item.token_type != TokenType::Enumeration { return 
                        Err("Structs/Enums have to be defined after modules."
                            .into()) }

                content_lines.push(format!("    pub {} {} {{", 
                    <TokenSet as RustGen>::
                    generate_keyword_from_token_type(&secondary_item),
                    secondary_item.token_name));

                // Loop through the inner types of a struct/enum.
                secondary_item.childs.iter().enumerate().for_each(|(pos, item)| {
                    let colon = pos < secondary_item.childs.len();

                    if secondary_item.token_type == TokenType::Structure {
                        content_lines.push(format!("        {}{}",
                            <TokenSet as RustGen>::build_type_declaration(
                                item, &reusability),
                            if colon { "," } else { "" }));
                    } else {
                        content_lines.push(format!("        {}{}",
                            item.custom_token_type.as_ref().unwrap(),
                            if colon { "," } else { "" }));
                    }
                });
                content_lines.push("    }".into());
            }
            content_lines.push("}".into());
        }

        let content_lines = content_lines.join("\n");

        debug!("Generated content:\n{}", content_lines);

        try_write_bytes_to_file(output_path, content_lines.as_bytes())
    }
}


#[cfg(feature = "go-gen")]
use super::generator::GoGen;
#[cfg(feature = "go-gen")]
impl GoGen for TokenSet {
    fn generate_keyword_from_token_type(token: &TokenSet) -> String {
        match token.token_type {
            TokenType::Char => { "rune" }
            TokenType::Bool => { "bool" }
            TokenType::IntU8 => { "uint8" }
            TokenType::IntI8 => { "int8" }
            TokenType::Structure => { "struct" }
            TokenType::String => { "string" }
            TokenType::IntU16 => { "uint16" }
            TokenType::IntU32 => { "uint32" }
            TokenType::IntU64 => { "uint64" }
            TokenType::IntI16 => { "int16" }
            TokenType::IntI32 => { "int32" }
            TokenType::IntI64 => { "int64" }
            TokenType::Custom => { return token.clone().
                custom_token_type.unwrap() }
            _ => { "" }
        }.into()
    }

    fn build_type_declaration(
        token: &TokenSet, reusability: &ReusableDeclarations) -> String {
        // Due to enums not existing as a proper thing, a custom type pointing
        // to an enum cannot exist. It's thus mapped as a `string`. 
        if token.parameters.contains(&TokenParameter::LocalType) {
            // Determines if the custom token type is present inside the enum
            // flatten reusability list, if it's present it means the custom type
            // maps to an enum. 
        
            let search_output = 
                reusability.find_declaration_descriptor_with_declaration_name(
                    token.custom_token_type.clone().unwrap());

            if search_output.is_some() { return "string".into() }
        }


       let mut output_type = format!("{}", <TokenSet as GoGen>::
                                  generate_keyword_from_token_type(token));

       if token.parameters.contains(&TokenParameter::Optional) {
            output_type = format!("*{}", output_type);
       }

        if token.parameters.contains(&TokenParameter::Floated) {
            match token.token_type {
                TokenType::IntU32 | TokenType::IntI32| TokenType::IntU64 
                    | TokenType::IntI64 => {
                        output_type = output_type.replace("u", "")
                            .replace("int", "float");
                    }
                _ => { 
                    warn!("Go: Floated applies only to [u|i][32|64] types.") 
                } 
            }
        }

       if token.parameters.contains(&TokenParameter::Pointer) ||
           token.parameters.contains(&TokenParameter::Reference) {
            warn!("Go: pointer and ref does the same thing.");
            
            output_type = format!("*{}", output_type);
        }

       if token.parameters.contains(&TokenParameter::Vector) {
            output_type = format!("[]{}", output_type);
       }

       return format!("{}", output_type) 
    }

    fn produce_go_build_in_single_file(
            source: Vec<TokenSet>,
            reusability: ReusableDeclarations,
            output_path: String,
            output_package_name: String) 
            -> Result<(), String> {
        // Content is generated line by line and put there before being joined
        // and saved.
        let mut content_lines: Vec<String> = vec![];

        warn!("Go: due to language limitations, modules grouping is ignored.");
        warn!("Go: due to language limitations, enums are set to constants.");

        content_lines.push(format!("package {}\n", output_package_name));

        for root_item in source {
            // Loops through the Module's childs.
            for secondary_item /* Such as struct or enum. */ in 
                root_item.childs {
                if secondary_item.token_type != TokenType::Structure && 
                    secondary_item.token_type != TokenType::Enumeration{ return 
                        Err("Structs have to be defined before types."
                           .into()) }

                if secondary_item.token_type == TokenType::Structure {
                    content_lines.push(format!("type {}{} {} {{", 
                        root_item.token_name,
                        secondary_item.token_name,
                        <TokenSet as GoGen>::
                        generate_keyword_from_token_type(&secondary_item)));
                }
                // Loops through the inner types of the Struct/Enum.
                
                // For enums, the secondary item is not used to group types
                // but as a part of the produced item title.

                for inner_item in secondary_item.childs.iter().enumerate() {
                    if secondary_item.token_type == TokenType::Structure {
                        content_lines.push(format!("    {} {}",
                            inner_item.1.token_name,
                            <TokenSet as GoGen>::build_type_declaration(
                              inner_item.1, &reusability)));
                    } else {
                        let variable_name = 
                            format!("{}{}{}",
                                root_item.token_name,
                                secondary_item.token_name,
                                <TokenSet as GoGen>::build_type_declaration(
                                    inner_item.1, &reusability)
                                );
                        content_lines.push(
                            format!("const {} = \"{}-{}\"",
                            variable_name, variable_name, inner_item.0));
                    }
                }

                if secondary_item.token_type == TokenType::Structure {
                    content_lines.push("}".into());
                }
                 
            }
        }

        let content_lines = content_lines.join("\n");

        debug!("Generated content:\n{}", content_lines);

        try_write_bytes_to_file(output_path, content_lines.as_bytes())
    }
}


#[cfg(feature = "ts-gen")]
use super::generator::TSGen;
#[cfg(feature = "ts-gen")]
impl TSGen for TokenSet {
    fn generate_keyword_from_token_type(token: &TokenSet) -> String {
        match token.token_type {
            TokenType::String | TokenType::Char => { "string" }
            TokenType::IntU8 | TokenType::IntI8 | TokenType::IntU16
                | TokenType::IntI16 | TokenType::IntU32 | TokenType::IntI32
                | TokenType::IntU64 | TokenType::IntI64 => { "number" }
            TokenType::Module => { "namespace" }
            TokenType::Structure => { "interface" }
            TokenType::Enumeration => { "enum" }
            TokenType::Bool => { "boolean" }
            TokenType::Custom => { return token.custom_token_type.clone()
                .unwrap() }
        }.into()
    }

    fn build_type_declaration(
        token: &TokenSet, reusability: &ReusableDeclarations) -> String {
       let mut output_type = format!("{}", <TokenSet as TSGen>::
                                     generate_keyword_from_token_type(token));

        if token.parameters.contains(&TokenParameter::LocalType) {
            let reusable_data = reusability.
                    find_declaration_descriptor_with_declaration_name( 
                        token.custom_token_type.clone().unwrap());

            debug!("Found a token with the `local` parameter.");

            if reusable_data.is_some() {
                output_type = <ItemDeclarationDescriptor as TSReusability>
                    ::produce_reusable_statement_from_struct_or_enum_token(
                        reusable_data.unwrap());
            } 
        }

        if token.parameters.contains(&TokenParameter::Vector) {
            output_type = format!("{}[]", output_type);
        } 

        if token.parameters.contains(&TokenParameter::Optional) { 
            format!("{}?: {};", 
                    token.token_name.to_lower_camel_case(), output_type)
        } else {
            format!("{}: {};",
                    token.token_name.to_lower_camel_case(), output_type)
        }
    }
    
    fn produce_ts_build_in_single_file(
            source: Vec<TokenSet>, 
            reusability: ReusableDeclarations,
            output_path: String)
            -> Result<(), String> {
       // Content is generated line by line and is put here before being joined
       // at save time.
       let mut content_lines: Vec<String> = vec![];

       warn!("Typescript: Integers and floats precision is lost.");
       warn!("Typescript: Pointers and references are not used.");

       for root_item in source {
            // When generating TypeScript code, a root_item always has to be a
            // module.
            if root_item.token_type != TokenType::Module { return 
                Err("Modules have to be declared before anything.".into()) }

            content_lines.push(format!("export namespace {} {{", 
                                       root_item.token_name));

            // Loops through the Module's childs.
            for secondary_item in root_item.childs {
                if secondary_item.token_type != TokenType::Structure &&
                    secondary_item.token_type != TokenType::Enumeration { return
                        Err("Structs/Enums have to be defined after modules."
                            .into())}

                content_lines.push(format!("    export {} {} {{",
                    <TokenSet as TSGen>::
                    generate_keyword_from_token_type(&secondary_item),
                    secondary_item.token_name));

                // Loops through the inner types of the Struct/Enum
                for inner_item in secondary_item.childs {
                    // If the inner item is a child of an enum, only the custom
                    // type is used.
                    if secondary_item.token_type == TokenType::Structure {
                        content_lines.push(format!("        {}",
                            <TokenSet as TSGen>::build_type_declaration(
                                &inner_item, &reusability)));
                    } else {
                        content_lines.push(format!("        {},",
                            inner_item.custom_token_type.unwrap()));
                    }
                }
                content_lines.push("    }".into());
            }
            content_lines.push("}".into());
        }

       let content_lines = content_lines.join("\n");

       debug!("Generated content:\n{}", content_lines);

       try_write_bytes_to_file(output_path, content_lines.as_bytes())
    }
}
