use crate::engine::tokenizer::{TokenType, TokenParameter};

// Ease the usage of multiple conditions by resuming it to a single function 
// call. It works by checking if `val` == `equals`, and setting `&mut target` 
// to`on_true`.
pub struct RuleSet<T, V: PartialEq> {
    pub value: Option<T>,
    pub to_compare: V
}


// Build a RuleSet from any type.
pub fn make_rule_set<T, V: PartialEq>(to_compare: V) -> RuleSet::<T, V> {
    RuleSet { value: None, to_compare }
}

impl RuleSet<String, String> {
    #[allow(dead_code)]
    pub fn exec_rule(&mut self, if_true: String, equals: String) -> () {
        if self.to_compare == equals { 
            self.value = Some(if_true);
        }
    }
}

impl RuleSet<TokenType, String> {
    #[allow(dead_code)]
    pub fn exec_rule(&mut self, if_true: TokenType, equals: String) -> () {
        if self.to_compare == equals {
            self.value = Some(if_true);
        }
    }
}

impl RuleSet<TokenParameter, String> {
    #[allow(dead_code)]
    pub fn exec_rule(&mut self, if_true: TokenParameter, equals: String) -> () {
        if self.to_compare == equals {
            self.value = Some(if_true);
        }
    }
}
