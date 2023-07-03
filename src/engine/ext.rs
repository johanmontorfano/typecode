use crate::utils::file::try_write_bytes_to_file;
use crate::{debug, warn};
use super::tokenizer::{TokenSet, TokenType, TokenParameter};
use super::generator::{RustGen, GoGen, TSGen};

impl RustGen for TokenSet {
    fn generate_keyword_from_token_type(
            token: &TokenSet) -> String {
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
            token: &TokenSet) -> String {
        let mut output_type = format!("{}", <TokenSet as RustGen>::
                                  generate_keyword_from_token_type(token));

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

       return format!("pub {}: {},", token.token_name, output_type)
    }

    fn produce_rs_build_in_single_file(
            source: Vec<TokenSet>, 
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
                
                // Loops through the inner types of the Struct/Enum.
                for inner_item in secondary_item.childs {
                    // If the inner item is a child of an enum, only the 
                    // custom type is used.
                    if secondary_item.token_type == TokenType::Structure {
                        content_lines.push(format!("        {}",
                            <TokenSet as RustGen>::build_type_declaration(
                                &inner_item)));
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

    fn build_type_declaration(token: &TokenSet) -> String {
       let mut output_type = format!("{}", <TokenSet as GoGen>::
                                  generate_keyword_from_token_type(token));

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
            source: Vec<TokenSet>, output_path: String) 
            -> Result<(), String> {
        // Content is generated line by line and put there before being joined
        // and saved.
        let mut content_lines: Vec<String> = vec![];

        warn!("Go: due to language limitations, modules grouping is ignored.");
        warn!("Go: due to language limitations, enums are set to constants.");
        
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
                              inner_item.1)));
                    } else {
                        let variable_name = 
                            format!("{}{}{}",
                                root_item.token_name,
                                secondary_item.token_name,
                                <TokenSet as GoGen>::build_type_declaration(
                                    inner_item.1)
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

    fn build_type_declaration(token: &TokenSet) -> String {
       let mut output_type = format!("{}", <TokenSet as TSGen>::
                                     generate_keyword_from_token_type(token));

       if token.parameters.contains(&TokenParameter::Vector) {
            output_type = format!("{}[]", output_type);
       } 

       return format!("{}: {};", token.token_name, output_type);
    }
    
    fn produce_ts_build_in_single_file(
            source: Vec<TokenSet>, output_path: String)
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
                                &inner_item)));
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
