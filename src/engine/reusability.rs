use super::tokenizer::{TokenType, TokenSet};
use crate::debug;

#[derive(Clone, Debug)]
pub struct ItemDeclarationDescriptor {
    pub module_name: String,
    pub declaration_name: String,
    pub declaration_type: TokenType
}

// Contains a flatten tree of items declarations.
#[derive(Clone, Debug)]
pub struct ReusableDeclarations {
    pub structs: Vec<ItemDeclarationDescriptor>,
    pub enums: Vec<ItemDeclarationDescriptor>
}

impl ReusableDeclarations {
    // Build the flatten tree from modules token sets.
    pub fn from_token_sets_vec(source: Vec<TokenSet>) -> Self {
        let mut output = ReusableDeclarations { structs: vec![], enums: vec![] };

        source.iter().for_each(|token| {
            // Doesn't process the token if it's not a module.
            if token.token_type == TokenType::Module {
                for child in token.childs.clone() {
                    // Builds the declaration descriptor before further checks.
                    let declaration_descriptor = ItemDeclarationDescriptor {
                        module_name: token.token_name.clone(),
                        declaration_name: child.token_name,
                        declaration_type: child.token_type
                    };

                    // Checks it for being either a struct or an enum because
                    // nothing else should be a child of a module.
                    match declaration_descriptor.declaration_type {
                        TokenType::Structure => { 
                            output.structs.push(declaration_descriptor) },
                        TokenType::Enumeration => {
                            output.enums.push(declaration_descriptor) },
                        _ => ()
                    }
                }
            }
        });

       debug!("Processed reusable declarations: {:#?}", output.clone());

       return output;
    }

    // Searches a `ItemDeclarationDescriptor` by `declaration_name`.
    pub fn find_declaration_descriptor_with_declaration_name(
        &self, declaration_name: String) -> Option<&ItemDeclarationDescriptor> {
        let struct_search = self.structs.iter()
            .find(|item| item.declaration_name == declaration_name);
        let enum_search = self.enums.iter()
            .find(|item| item.declaration_name == declaration_name);

        if struct_search.is_some() { return struct_search }
        else if enum_search.is_some() { return enum_search }
        else { None } 
    }
}
