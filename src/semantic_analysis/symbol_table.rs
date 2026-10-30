//! # Symbol Table
//!
//! The symbol table consists of all variables and objects defined throughout the entire program,
//! so when we reference a variable or object, applying Java's scoping rules, we know exactly
//! which entry in our symbol table is being referred to.

use std::collections::HashMap;

use crate::semantic_analysis::error_log::ErrorLog;

pub struct ClassInfo {
    id: String,
    super_id: Option<String>,
    fields: Vec<VariableInfo>,
    methods: Vec<MethodInfo>,
}

impl ClassInfo {
    pub fn new(id: String, super_id: Option<String>) -> ClassInfo {
        ClassInfo {
            id,
            super_id,
            fields: Vec::new(),
            methods: Vec::new(),
        }
    }
}

pub struct MethodInfo {
    id: String,
    return_type: TypeInfo,
    parameters: Vec<VariableInfo>,
    variables: Vec<VariableInfo>,
}

pub struct VariableInfo {
    id: String,
    ty: TypeInfo,
}

pub enum TypeInfo {
    Boolean,
    Int,
    IntArray,
}

///
pub struct SymbolTable {
    /// Maps the names of classes to the class information.
    class_table: HashMap<String, ClassInfo>,

    /// Used to store errors during the name resolution and type checking passes.
    error_log: ErrorLog,
}

impl SymbolTable {
    pub fn new(error_log: ErrorLog) -> SymbolTable {
        SymbolTable {
            class_table: HashMap::new(),
            error_log,
        }
    }

    pub fn add_class(&mut self, id: String, super_id: Option<String>) {
        // if self.class_table.contains_key(&id) {
        //     self.error_log
        //         .add_error(TypeError::DuplicateIdentifier, span);
        //     return;
        // }

        todo!()
    }
}
