use crate::{utils::conditions::make_rule_set, debug};

// Type of the token.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Module,
    Structure,
    Enumeration,
    String,
    Char,
    IntU8,
    IntU16,
    IntU32,
    IntU64,
    IntI8,
    IntI16,
    IntI32,
    IntI64,
    Bool,
    Custom
}

// Additional tokens that can help define the usage of the token.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenParameter {
    Floated,
    Pointer,
    Reference,
    Vector
}

#[derive(Debug, Clone)]
pub struct TokenSet {
    pub token_type: TokenType,
    pub custom_token_type: Option<String>,
    pub parameters: Vec<TokenParameter>,
    pub token_name: String,
    pub childs: Vec<TokenSet>
}

impl TokenSet {
    // Builds a `TokenSet` from a line.
    pub fn token_set_from_string(line: String) -> Option<Self> {
        // Lines with too few characters are ignored.
        if line.trim().len() < 2 { return None; }

        let mut tokens = line.trim().split(" ").map(|i| i.to_string())
            .collect::<Vec<String>>();
   
        // Fails if the set of tokens is not long enough.
        if tokens.len() < 1 {
            panic!("Too few arguments on line: {}", tokens.join(" "));

        }

        // Build a ruleset to find which token is used on this line.
        let mut token_type = make_rule_set::<TokenType, String>(
            tokens.get(0).unwrap().clone());
        
        token_type.exec_rule(TokenType::Module,      "module".into());
        token_type.exec_rule(TokenType::Structure,   "struct".into());
        token_type.exec_rule(TokenType::Enumeration, "enum".into());
        token_type.exec_rule(TokenType::String,      "string".into());
        token_type.exec_rule(TokenType::Char,        "char".into());
        token_type.exec_rule(TokenType::IntU8,       "int_u8".into());
        token_type.exec_rule(TokenType::IntU16,      "int_u16".into());
        token_type.exec_rule(TokenType::IntU32,      "int_u32".into());
        token_type.exec_rule(TokenType::IntU64,      "int_u64".into());
        token_type.exec_rule(TokenType::IntI8,       "int_i8".into());
        token_type.exec_rule(TokenType::IntI16,      "int_i16".into());
        token_type.exec_rule(TokenType::IntI32,      "int_i32".into());
        token_type.exec_rule(TokenType::IntI64,      "int_i64".into());        
        token_type.exec_rule(TokenType::Bool,        "bool".into());

        // Determines if there are token parameters present on the line.
        let mut token_parameters: Vec<TokenParameter> = vec![];
        
        if tokens.len() > 2 {
            tokens.iter().for_each(|tok| {
                let mut rule_set = make_rule_set::
                    <TokenParameter, String>(tok.clone());
                
                rule_set.exec_rule(TokenParameter::Vector,    "vec".into());
                rule_set.exec_rule(TokenParameter::Floated,   "floated".into());
                rule_set.exec_rule(TokenParameter::Pointer,   "pointer".into());
                rule_set.exec_rule(TokenParameter::Reference, "ref".into());

                // If a rule worked, the result is pushed into `
                // token_parameters`.
                if rule_set.value.is_some() { 
                    token_parameters.push(rule_set.value.unwrap()); 
                }
            });
        };

        // As the name of the Token should always be at the end of the line,
        // we extract the last entry from the splitted line as the Token Name.
        let token_name = tokens.pop().unwrap(); 

        // If the TokenType value is None, it means that a custom type is used,
        // thus should be set.
        let final_token_type = token_type.value.unwrap_or(TokenType::Custom);
        let custom_token_type = if final_token_type == TokenType::Custom {
            Some(token_type.to_compare) } else { None };

        debug!("Build item data: {} {:?}", token_name, final_token_type);

        return Some(Self {
            token_type: final_token_type,
            custom_token_type,
            parameters: token_parameters,
            token_name,
            childs: vec![]
        });
    }

    // Group tokens together in `childs` according to Token hierarchy:
    // - Modules
    //      - Structs / Enums
    //          - Types
    pub fn apply_hierarchy_rules(
        tokens_vec: Vec<TokenSet>) -> Vec<TokenSet> {
        let mut output: Vec<TokenSet> = vec![]; 

        tokens_vec.iter().for_each(|token| {
            match token.token_type {
                TokenType::Module => { 
                    output.push(token.clone());
                }
                TokenType::Structure | TokenType::Enumeration => {
                    if output.len() < 1 {
                        panic!("Struct/enum have to be defined under a module.");
                    }
               
                    // Push the struct/enum at the top of the module.
                    output.last_mut().unwrap()
                        .childs.push(token.clone());
                },
                _ => {
                    // Push the type at the top of the last struct pushed into
                    // the module.
                    output.last_mut().unwrap()
                        .childs.last_mut().unwrap()
                        .childs.push(token.clone());
                }
            }; 
        });
        
        return output;
    }
}
