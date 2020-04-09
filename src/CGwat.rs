/* 
Code generator for WebAssembly 
*/

use std::sync::Mutex;

use crate::SC;
use crate::ST;

// mutables (globals)
const TIMES: i32 = 1;
const DIV: i32 = 2;
const MOD: i32 = 3;
const AND: i32 = 4;
const PLUS: i32 = 5;
const MINUS: i32 = 6;
const OR: i32 = 7;
const EQ: i32 = 8;
const NE: i32 = 9;
const LT: i32 = 10;
const GT: i32 = 11;
const LE: i32 = 12;
const GE: i32 = 13;
const NOT: i32 = 24;

// immutables (globals)
static mut curlev: i32 = 0;
static mut memsize: i32 = 0;

// our global asm array which will be passed around from fn to fn
lazy_static! {
    static ref ASM: Mutex<Vec<String>> = Mutex::new(vec![]);
}

// push WebAssembly code string to global asm array
fn asm_push(a: String) {
    ASM.lock().unwrap().push(a);
}

pub fn asm_clear() {
    ASM.lock().unwrap().clear();
}

pub fn genProgStart() {
    asm_push(r"(module".to_string());
    asm_push(r#"(import "P0lib" "write" (func $write (param i32)))"#.to_string());
    asm_push(r#"(import "P0lib" "writeln" (func $writeln))"#.to_string());
    asm_push(r#"(import "P0lib" "read" (func $read (result i32)))"#.to_string());
}

/*
genGlobalVars(sc, start):
allocates a global WebAssembly variable by the same name,
if the type is Int or Bool, or reserves space in the memory, if the type is Array, Record
*/
// start --> int
// sc --> Vector of Symbol structs
// returns size of globalvars for P0
pub fn genGlobalVars(sc: &mut Vec<ST::Symbol>, start: i32) -> i32 {
    for i in sc.iter(){
        // println!("top scope: {}, tpname {}", i.type_name, i.tp.type_name);
        if i.type_name == "Var" {
            if i.tp.type_name == "Int" || i.tp.type_name == "Bool"{
                asm_push(
                    (r"(global $".to_owned() + &i.name + " (mut i32) i32.const 0)")
                        .to_string(),
                );
            }
            
        }
    }
    //         // } else if (sc[i as usize].type_name == "Array")
    //         //     || (sc[i as usize].type_name == "Record")
    //         // {
    //         //     sc[i as usize].lev = -2;
    //         //     unsafe {
    //         //         sc[i as usize].adr = memsize;
    //         //     }
    //         //     unsafe {
    //         //         memsize = memsize + sc[i as usize].tp.size;
    //         //     }
    //         // } else {
    //         //     SC::mark(r"WASM: type?".to_string());
    //         // }
    //     }
    // }
    return unsafe { memsize };
}

// start --> int
// sc --> Vector of Symbol structs
pub fn genLocalVars(sc: &mut Vec<ST::Symbol>, start: i32) {
    for i in start..sc.len() as i32 {
        if sc[i as usize].type_name == "Var".to_string() {
            if (sc[i as usize].tp.type_name == "Int".to_string())
                || (sc[i as usize].tp.type_name == "Bool".to_string())
            {
                asm_push((r"(local $".to_owned() + &sc[i as usize].name + r" i32)").to_string());
            } else if (sc[i as usize].type_name == "Array")
                || (sc[i as usize].type_name == "Record")
            {
                SC::mark(r"WASM: no local arrays, records".to_string());
            } else {
                SC::mark(r"WASM: type?".to_string());
            }
        }
    }
}

// loadItem(x) generates code for loading x on the expression stack
pub fn loadItem(x: &ST::Symbol) {
    if x.type_name == "Var".to_string() {
        if x.lev == 0 {
            asm_push((r"global.get $".to_owned() + &x.name).to_string());
        } else if unsafe { x.lev == curlev } {
            asm_push((r"local.get $".to_owned() + &x.name).to_string());
        } else if x.lev == -2 {
            asm_push((r"i32.const ".to_owned() + &x.adr.to_string()).to_string());
            asm_push(r"i32.load".to_string());
        } else if x.lev != -1 {
            SC::mark(r"WASM: var level!".to_string());
        }
    } else if x.type_name == "Ref".to_string() {
        if x.lev == -1 {
            asm_push(r"i32.load".to_string());
        } else if unsafe { x.lev == curlev } {
            asm_push((r"local.get $".to_owned() + &x.name).to_string());
            asm_push(r"i32.load".to_string());
        } else {
            SC::mark(r"WASM: ref level!".to_string());
        }
    } else if x.type_name == "Const" {
        asm_push((r"i32.const ".to_owned() + &x.val.to_string()).to_string());
    }
}

pub fn genVar(x: ST::Symbol) -> ST::Symbol {
    let mut y = ST::Symbol {
        ..Default::default()
    };
    if 0 < x.lev {
        if unsafe { x.lev < curlev } {
            SC::mark(r"WASM: level!".to_string());
        }
    }
    if x.type_name == "Ref".to_string() {
        y.type_name = "Ref".to_string();
        y.lev = x.lev;
        y.name = x.name;
    } else if x.type_name == "Var".to_string() {
        y.type_name = "Var".to_string();
        y.lev = x.lev;
        y.name = x.name;
        if x.lev == -2 {
            y.adr = x.adr;
        }
    }
    y.tp = x.tp;
    y
}

pub fn genConst(x: ST::Symbol) -> ST::Symbol {
    x
}

pub fn genBool(mut b: ST::Symbol) -> ST::Symbol {
    b.size = 1;
    b
}

pub fn genInt(mut i: ST::Symbol) -> ST::Symbol {
    i.size = 4;
    i
}

pub fn genRec(mut r: ST::Symbol) -> ST::Symbol {
    let mut s = 0;
    if r.type_name == "Record" {
        for f in 0..r.fields.len() {
            r.fields[f].offset = s;
            s = s + r.fields[f].tp.size;
        }
        r.size = s;
    }
    r
}

pub fn genArray(mut a: ST::Symbol) -> ST::Symbol {
    if a.type_name == "Array" {
        a.size = a.length * a.base.size;
    }
    a
}

pub fn genUnaryOp(op: i32, mut x: ST::Symbol) -> ST::Symbol {
    loadItem(&x);
    // let mut x = x;
    if op == MINUS {
        asm_push(r"i32.const -1".to_string());
        asm_push(r"i32.mul".to_string());
        // x = ST::Var(ST::Int);
        x = ST::Symbol {
            type_name: "Var".to_string(),
            tp: ST::PrimitiveTypes {
                type_name: "Int".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        x.lev = -1;
    } else if op == NOT {
        asm_push(r"i32.eqz".to_string());
        // x = ST::Var(ST::Bool);
        x = ST::Symbol {
            type_name: "Var".to_string(),
            tp: ST::PrimitiveTypes {
                type_name: "Bool".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        x.lev = -1;
    } else if op == AND {
        asm_push(r"if (result i32)".to_string());
        x = ST::Symbol {
            type_name: "Var".to_string(),
            tp: ST::PrimitiveTypes {
                type_name: "Bool".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        x.lev = -1;
    } else if op == OR {
        asm_push(r"if (result i32)".to_string());
        asm_push(r"i32.const 1".to_string());
        asm_push(r"else".to_string());
        x = ST::Symbol {
            type_name: "Var".to_string(),
            tp: ST::PrimitiveTypes {
                type_name: "Bool".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        x.lev = -1;
    } else {
        SC::mark(r"WASM: unary operator?".to_string());
    }
    return x;
}

pub fn genBinaryOp(op: i32, mut x: ST::Symbol, y: ST::Symbol) -> ST::Symbol {
    if op == PLUS || op == MINUS || op == TIMES || op == DIV || op == MOD {
        loadItem(&x);
        loadItem(&y);
        match op {
            TIMES => asm_push(r"i32.mul".to_string()),
            MINUS => asm_push(r"i32.sub".to_string()),
            PLUS => asm_push(r"i32.add".to_string()),
            DIV => asm_push(r"i32.div_s".to_string()),
            MOD => asm_push(r"i32.rem_s".to_string()),
            _ => asm_push(r"\n".to_string()),
        }
        x = ST::Symbol {
            type_name: "Var".to_string(),
            tp: ST::PrimitiveTypes {
                type_name: "Int".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        x.lev = -1;
    } else if op == AND {
        loadItem(&y);
        asm_push(r"else".to_string());
        asm_push(r"i32.const 0".to_string());
        asm_push(r"end".to_string());
        // x = ST::Var(ST::Bool);
        x = ST::Symbol {
            type_name: "Var".to_string(),
            tp: ST::PrimitiveTypes {
                type_name: "Bool".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        x.lev = -1;
    } else if op == OR {
        loadItem(&y);
        asm_push(r"end".to_string());
        x = ST::Symbol {
            type_name: "Var".to_string(),
            tp: ST::PrimitiveTypes {
                type_name: "Bool".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        x.lev = -1;
    } else {
        // do nothing
    }
    x
}

pub fn genRelation(op: i32, mut x: ST::Symbol, y: ST::Symbol) -> ST::Symbol {
    loadItem(&x);
    loadItem(&y);
    match op {
        EQ => asm_push(r"i32.eq".to_string()),
        NE => asm_push(r"i32.ne".to_string()),
        LT => asm_push(r"i32.lt_s".to_string()),
        GT => asm_push(r"i32.gt_s".to_string()),
        LE => asm_push(r"i32.le_s".to_string()),
        GE => asm_push(r"i32.ge_s".to_string()),
        _ => asm_push(r"\n".to_string()),
    }
    x = ST::Symbol {
        type_name: "Var".to_string(),
        tp: ST::PrimitiveTypes {
            type_name: "Bool".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
    x.lev = -1;
    x
}

pub fn genSelect(mut x: ST::Symbol, f: ST::Field) -> ST::Symbol {
    if x.type_name == "Var".to_string() {
        x.adr += f.offset;
    } else if x.type_name == "Ref".to_string() {
        if x.lev > 0 {
            asm_push((r"local.get $".to_owned() + &x.name).to_string());
        }
        asm_push((r"i32.const ".to_owned() + &f.offset.to_string()).to_string());
        asm_push(r"i32.add".to_string());
        x.lev = -1;
    }
    x.tp.type_name = f.tp.type_name;
    x
}

pub fn genIndex(mut x: ST::Symbol, y: ST::Symbol) -> ST::Symbol {
    if x.type_name == "Var".to_string() {
        if y.type_name == "Const" {
            x.adr += (y.val - x.tp.lower) * x.tp.base.size;
        // x.tp = x.tp.base;
        } else {
            loadItem(&y);
            if x.tp.lower != 0 {
                asm_push((r"i32.const ".to_owned() + &x.tp.lower.to_string()).to_string());
                asm_push(r"i32.sub".to_string());
            }
            asm_push((r"i32.const ".to_owned() + &x.tp.base.size.to_string()).to_string());
            asm_push(r"i32.mul".to_string());
            asm_push((r"i32.const ".to_owned() + &x.adr.to_string()).to_string());
            asm_push(r"i32.add".to_string());
            // x = ST::Ref(x.tp.base);
            x.lev = -1;
        }
    } else {
        if unsafe { x.lev == curlev } {
            loadItem(&x);
            x.lev = -1;
        }
        if y.type_name == "Const" {
            asm_push(
                (r"i32.const ".to_owned() + &((y.val - x.tp.lower) * x.tp.base.size).to_string())
                    .to_string(),
            );
            asm_push(r"i32.add".to_string());
        } else {
            loadItem(&y);
            asm_push((r"i32.const ".to_owned() + &x.tp.lower.to_string()).to_string());
            asm_push(r"i32.sub".to_string());
            asm_push((r"i32.const ".to_owned() + &x.tp.base.size.to_string()).to_string());
            asm_push(r"i32.mul".to_string());
            asm_push(r"i32.add".to_string());
        }
        // x.tp = x.tp.base;
    }
    x
}

pub fn genAssign(x: ST::Symbol, y: ST::Symbol) {
    if x.type_name == "Var".to_string() {
        if x.lev == -2 {
            asm_push((r"i32.const ".to_owned() + &x.adr.to_string()).to_string());
        }
        loadItem(&y);
        if x.lev == 0 {
            asm_push((r"global.set $".to_owned() + &x.name).to_string());
        } else if unsafe { x.lev == curlev } {
            asm_push((r"local.set $".to_owned() + &x.name).to_string());
        } else if x.lev == -2 {
            asm_push(r"i32.store".to_string());
        } else {
            SC::mark(r"WASM: level!".to_string());
        }
    } else if x.type_name == "Ref".to_string() {
        if unsafe { x.lev == curlev } {
            asm_push((r"local.get $".to_owned() + &x.name).to_string());
        }
        loadItem(&y);
        asm_push(r"i32.store".to_string());
    } else {
        println!("unable to assign (cgwat)");
    }
}

pub fn genProgEntry(_ident: i32) {
    asm_push(r"(func $program".to_string());
}

pub fn genProgExit() -> String {
    let mut _mem = unsafe { memsize };
    unsafe {
        asm_push(
            (")\n(memory ".to_owned()
                + &(memsize / i32::pow(2, 16) + 1).to_string()
                + ")\n(start $program)\n)")
                .to_string(),
        );
    }

    let asm = ASM.lock().unwrap();

    let masterString = asm.join("\n");
    masterString.to_string();
    masterString
}

pub fn genProcStart(ident: i32, fp: &mut Vec<ST::Symbol>) {
    if unsafe { curlev > 0 } {
        SC::mark(r"WASM: no nested procedures".to_string());
    }
    unsafe {
        curlev += 1;
    }

    let mut middleString: String = "".to_string();
    for e in 0..fp.len() {
        middleString = middleString.to_owned()
            + &"(param $".to_owned()
            + &fp[e].name.to_owned()
            + &" i32) ".to_owned();
    }
    asm_push((r"(func $".to_owned() + &ident.to_string() + " " + &middleString + "").to_string());

    for p in 0..fp.len() {
        if fp[p].tp.type_name == "Int" || fp[p].tp.type_name == "Bool" {
            if fp[p].type_name == "Ref".to_string() {
                SC::mark(r"WASM: only array and record reference parameters".to_string());
            } else if fp[p].tp.type_name == "Array".to_string()
                || fp[p].tp.type_name == "Record".to_string()
            {
                if fp[p].type_name == "Var".to_string() {
                    SC::mark(r"WASM: no structured value parameters".to_string());
                }
            }
        }
    }
}

pub fn genProcExit() {
    unsafe {
        curlev -= 1;
    }
    asm_push(r")".to_string());
}

pub fn genActualPara(ap: ST::Symbol, fp: ST::Symbol) {
    if fp.type_name == "Ref".to_string() {
        if ap.lev == -2 {
            asm_push((r"i32.const ".to_owned() + &ap.adr.to_string()).to_string());
        }
    } else if ap.type_name == "Var".to_string()
        || ap.type_name == "Ref".to_string()
        || ap.type_name == "Const"
    {
        loadItem(&ap);
    } else {
        SC::mark(r"unsupported parameter type".to_string());
    }
}

pub fn genCall(pr: ST::Symbol) {
    asm_push((r"call $".to_owned() + &pr.name.to_string()).to_string());
}

pub fn genRead(_x: ST::Symbol) {
    asm_push(r"call $read".to_string());
    let mut y = ST::Symbol {
        type_name: "Var".to_string(),
        tp: ST::PrimitiveTypes {
            type_name: "Int".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };
    y.lev = -1;
}

pub fn genWrite(x: ST::Symbol) {
    loadItem(&x);
    asm_push(r"call $write".to_string());
}

pub fn genWriteln() {
    asm_push(r"call $writeln".to_string());
}

pub fn genThen(x: ST::Symbol) -> ST::Symbol {
    loadItem(&x);
    asm_push(r"if".to_string());
    x
}

pub fn genIfThen() {
    asm_push(r"end".to_string());
}

pub fn genElse() {
    asm_push(r"else".to_string());
}

pub fn genIfElse() {
    asm_push(r"end".to_string());
}

pub fn genWhile() {
    asm_push(r"loop".to_string());
}

pub fn genDo(x: ST::Symbol) -> ST::Symbol {
    loadItem(&x);
    asm_push(r"if".to_string());
    x
}

pub fn genWhileDo() {
    asm_push(r"br 1".to_string());
    asm_push(r"end".to_string());
    asm_push(r"end".to_string());
}
