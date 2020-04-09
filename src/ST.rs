/* 
Symbol table implementation 
*/

use crate::SC;
use std::sync::Mutex;
use std::fmt;

// Added a primitive Types Struct to model base types Int and Bool that were in the python code.
// Needed to have these properties as an addition to the properties of the Symbol struct
// As an anlogue for Pythons type() function
#[derive(Clone)]
pub struct PrimitiveTypes {
    pub base: Base,
    pub lower: i32,
    pub size: i32,
    pub type_name: String,
    pub fields: Vec<Field>,
    pub length: i32,
}

// Default implementation so we do not have to fill out the fields of a pimitive type everytime we want to
// initialize one in rust
impl Default for PrimitiveTypes {
    fn default() -> PrimitiveTypes {
        PrimitiveTypes {
            base: Base {
                ..Default::default()
            },
            lower: 0,
            size: 0,
            type_name: "".to_string(),
            fields: vec![Field {
                ..Default::default()
            }],
            length: 0,
        }
    }
}

// Need a seperate struct for Primitive types with fields, (properties or arrays)
#[derive(Clone)]
pub struct FieldPrimitiveTypes {
    pub type_name: String,
    pub size: i32,
}
// need a default for these fields as well
impl Default for FieldPrimitiveTypes {
    fn default() -> FieldPrimitiveTypes {
        FieldPrimitiveTypes {
            type_name: "".to_string(),
            size: 0,
        }
    }
}
// Meant to be the records in arrays
#[derive(Clone)]
pub struct Field {
    pub name: String,
    pub size: i32,
    pub offset: i32,
    pub tp: FieldPrimitiveTypes,
}

impl Default for Field {
    fn default() -> Field {
        Field {
            name: "".to_string(),
            size: 0,
            offset: 0,
            tp: FieldPrimitiveTypes {
                ..Default::default()
            },
        }
    }
}
// A struct meant to be an analogue for Base size property
#[derive(Clone)]
pub struct Base {
    pub size: i32,
}

impl Default for Base {
    fn default() -> Base {
        Base { size: 0 }
    }
}


// The main data type, mean to be a replacement for the Var, Const, Type, Array, Record, Etc. classes in Python
// Needed to be one big struct to model everything as Rust requires strict typing so could not have it returning
// a vareity of classes/structs per function, so a Super Struct was created to bypass this issue.
#[derive(Clone)]
pub struct Symbol {
    pub name: String, // Name of the Var, Type, etc
    pub lev: i32, // Level in symbol table
    pub tp: PrimitiveTypes, // Primitive type struct such as Bool or Int
    pub val: i32, // Value, number for Ints, 0 for True, 1 for False for Bool
    pub par: Vec<String>, // Vector of Par
    pub base: Base, // Base struct type for size
    pub lower: i32, // i32 for lower
    pub length: i32, // i32 for length
    pub adr: i32, // i32 for address
    pub size: i32, // i32 for size in bytes the Symbol takes up
    pub fields: Vec<Field>, // Vector of fields if its an array or record

    // this will be the thing that distinguishes between
    // Const, Int, etc...
    pub type_name: String, // Type of Class in Python (Var, Const, Type, etc.)
}
//To print out our Symbol type for debugging reasons
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Symbol(name = {}, lev = {}, val = {}, type_name = {})", self.name, self.lev, self.val, self.type_name)
    }
}
// Default constructor so we do not have to fill out every field when makeing a new symbol
impl Default for Symbol {
    fn default() -> Symbol {
        Symbol {
            type_name: "".to_string(),
            name: "".to_string(),
            lev: 0,
            tp: PrimitiveTypes {
                ..Default::default()
            },
            val: 0,
            par: vec![],
            base: Base {
                ..Default::default()
            },
            lower: 0,
            length: 0,
            adr: 0,
            size: 0,
            fields: vec![],
        }
    }
}
// Basic getters and setters
impl Symbol {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
    fn get_name(&self) -> &String {
        return &self.name;
    }
    fn push(self, mut sym: Vec<Symbol>) {
        sym.push(self);
    }
}
// This is the Global symbol table vector. Needs to be wrapped in a lazy_static block  
// with a mutex lock as rust does not allow global variables
lazy_static! {
    static ref symTab: Mutex<Vec<Vec<Symbol>>> = Mutex::new(vec![vec![]]);
}
// Need a function to unlock the mutex to push a Scope vector onto the table
fn symTab_push(a: Vec<Symbol>) {
    symTab.lock().unwrap().push(a);
}
// Empties table if we need to
fn symTab_clear() {
    symTab.lock().unwrap().clear();
}
// Initialize function in P0 compiler
pub fn init() {
    symTab_clear();
    symTab_push(Vec::default());
}
// Prints symbol table by iterating through and printing the name of every Symbol
pub fn printSymTab() {
    println!("printing table");
    println!("-----------------");
    let mut sym = symTab.lock().unwrap();
    if sym.len() > 0 {
        for i in 0..sym.len() {
            for j in 0..sym[i].len() {
                println!("{}", sym[i][j].name);
            }
        }
    } else {
        println!("symbol table empty");
    }
    println!("-----------------");
    println!("Done printing table!!");
}
// Declares a new Symbol in the table
pub fn newDecl(name: String, mut entry: Symbol) {
    let mut table = &mut symTab.lock().unwrap();
    entry.lev = table.len() as i32 - 1;
    entry.name = name.to_string();
    for e in 0..table[0].len() {
        if table[0][e as usize].name == name.to_string() {
            SC::mark("multiple definition".to_string());
            return;
        }
    }
    table[0].push(entry);
}
// Finds a symbol in the table using a name
pub fn find(name: String) -> Symbol {
    let mut table = &mut symTab.lock().unwrap();
    for l in 0..table.len() {
        for e in 0..table[l].len() {
            if name == table[l][e].name {
                return table[l][e].clone();
            }
        }
    }
    return Symbol {
        type_name: "Const".to_string(),
        val: 0,
        ..Default::default()
    };
}
// Adds a new scope vector to the table
pub fn openScope() {
    let mut table = &mut symTab.lock().unwrap();
    table.insert(0, vec![]);
}
// Returns the top scope vector in the table
pub fn topScope() -> Vec<Symbol> {
    let mut table = &mut symTab.lock().unwrap();
    return table[0].clone();
}
// Removes the top scope from the table
pub fn closeScope() {
    let mut table = &mut symTab.lock().unwrap();
    table.remove(0);
}