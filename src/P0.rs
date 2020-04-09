/* 
Main P0 
*/

use std::sync::Mutex;

use std::collections::HashSet;

use crate::CGwat;
use crate::SC;
use crate::ST;

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
const PERIOD: i32 = 14;
const COMMA: i32 = 15;
const COLON: i32 = 16;
const RPAREN: i32 = 17;
const RBRAK: i32 = 18;
const OF: i32 = 19;
const THEN: i32 = 20;
const DO: i32 = 21;
const LPAREN: i32 = 22;
const LBRAK: i32 = 23;
const NOT: i32 = 24;
const BECOMES: i32 = 25;
const NUMBER: i32 = 26;
const IDENT: i32 = 27;
const SEMICOLON: i32 = 28;
const END: i32 = 29;
const ELSE: i32 = 30;
const IF: i32 = 31;
const WHILE: i32 = 32;
const ARRAY: i32 = 33;
const RECORD: i32 = 34;
const CONST: i32 = 35;
const TYPE: i32 = 36;
const VAR: i32 = 37;
const PROCEDURE: i32 = 38;
const BEGIN: i32 = 39;
const PROGRAM: i32 = 40;
const EOF: i32 = 41;

pub fn FIRSTFACTOR(i: i32) -> bool {
    return match i {
        IDENT => true,
        NUMBER => true,
        LPAREN => true,
        NOT => true,
        _ => false,
    };
}

pub fn FOLLOWFACTOR(i: i32) -> bool {
    return match i {
        TIMES => true,
        DIV => true,
        MOD => true,
        AND => true,
        OR => true,
        PLUS => true,
        MINUS => true,
        EQ => true,
        NE => true,
        LT => true,
        LE => true,
        GT => true,
        GE => true,
        COMMA => true,
        SEMICOLON => true,
        THEN => true,
        RPAREN => true,
        RBRAK => true,
        DO => true,
        PERIOD => true,
        END => true,
        _ => false,
    };
}

pub fn FIRSTEXPRESSION(i: i32) -> bool {
    return match i {
        PLUS => true,
        MINUS => true,
        IDENT => true,
        NUMBER => true,
        LPAREN => true,
        NOT => true,
        _ => false,
    };
}

pub fn FIRSTSTATEMENT(i: i32) -> bool {
    return match i {
        IDENT => true,
        IF => true,
        WHILE => true,
        BEGIN => true,
        _ => false,
    };
}

pub fn FOLLOWSTATEMENT(i: i32) -> bool {
    return match i {
        SEMICOLON => true,
        END => true,
        ELSE => true,
        _ => false,
    };
}

pub fn FIRSTTYPE(i: i32) -> bool {
    return match i {
        IDENT => true,
        RECORD => true,
        ARRAY => true,
        LPAREN => true,
        _ => false,
    };
}

pub fn FOLLOWTYPE(i: i32) -> bool {
    return match i {
        SEMICOLON => true,
        _ => false,
    };
}

pub fn FIRSTDECL(i: i32) -> bool {
    return match i {
        CONST => true,
        TYPE => true,
        VAR => true,
        PROCEDURE => true,
        _ => false,
    };
}

pub fn FOLLOWDECL(i: i32) -> bool {
    return match i {
        BEGIN => true,
        _ => false,
    };
}

pub fn FOLLOWPROCCALL(i: i32) -> bool {
    return match i {
        SEMICOLON => true,
        END => true,
        ELSE => true,
        _ => false,
    };
}

pub fn STRONGSYMS(i: i32) -> bool {
    return match i {
        CONST => true,
        TYPE => true,
        VAR => true,
        PROCEDURE => true,
        WHILE => true,
        IF => true,
        BEGIN => true,
        EOF => true,
        _ => false,
    };
}

pub fn selector(x2: ST::Symbol) -> ST::Symbol {
    // Need to copy over entire object otherwise run into 'value has been moved' error
    let mut x = ST::Symbol {
        type_name: x2.type_name,
        fields: x2.fields,
        name: x2.name,
        lev: x2.lev,
        tp: ST::PrimitiveTypes {
            base: x2.tp.base,
            lower: x2.tp.lower,
            size: x2.tp.size,
            type_name: x2.tp.type_name,
            fields: x2.tp.fields.to_vec(),
            length: x2.tp.length,
        },
        val: x2.val,
        par: x2.par,
        base: x2.base,
        lower: x2.lower,
        length: x2.length,
        adr: x2.adr,
        size: x2.size,
    };
    while SC::symbol() == PERIOD || SC::symbol() == LBRAK {
        if SC::symbol() == PERIOD {
            SC::getSym();
            if SC::symbol() == IDENT {
                if x.tp.type_name == "Record" {
                    for f in x2.tp.fields.iter() {
                        if f.name == SC::val_string() {
                            let mut newF = ST::Field {
                                name: f.name.to_string(),
                                size: f.size,
                                offset: f.offset,
                                tp: ST::FieldPrimitiveTypes {
                                    type_name: f.tp.type_name.to_string(),
                                    size: f.tp.size,
                                },
                            };

                            x = CGwat::genSelect(x, newF);
                            break;
                        } else {
                            SC::mark("not a field".to_string());
                        }
                    }
                    SC::getSym();
                } else {
                    SC::mark("Not a record".to_string());
                }
            } else {
                SC::mark("identifier expected".to_string());
            }
        } else {
            SC::getSym();
            let mut y = expression();
            if x.tp.type_name == "Array" {
                if y.tp.type_name == "Int" {
                    //TODO(KUMAIL): Figure out what 'lower' is
                    if y.type_name == "Const"
                        && (y.val < x.tp.lower || y.val >= x.tp.lower + x.tp.length)
                    {
                        SC::mark("index out of bounds".to_string());
                    } else {
                        x = CGwat::genIndex(x, y);
                    }
                } else {
                    SC::mark("index not integer".to_string());
                }
            } else {
                SC::mark("not an array".to_string());
            }
            if SC::symbol() == RBRAK {
                SC::getSym();
            } else {
                SC::mark("] expected".to_string());
            }
        }
    }
    return x;
}

pub fn factor() -> ST::Symbol {
    let mut x = ST::Symbol {
        ..Default::default()
    };
    if !FIRSTFACTOR(SC::symbol()) {
        // println!("292 {}", SC::symbol());
        // SC::mark("expression expected (1)".to_string());

        while !FIRSTFACTOR(SC::symbol()) && !FOLLOWFACTOR(SC::symbol()) && !STRONGSYMS(SC::symbol())
        {
            SC::getSym();
        }
    }

    if SC::symbol() == IDENT {
        x = ST::find(SC::val_string());
        if x.type_name == "Var" || x.type_name == "Ref" {
            x = CGwat::genVar(x);
            SC::getSym();
        } else if x.type_name == "Const" {
            x = ST::Symbol {
                type_name: "Const".to_string(),
                val: x.val,
                tp: x.tp,
                ..Default::default()
            };
            x = CGwat::genConst(x);
            SC::getSym();
        } else {
            SC::mark("expression expected".to_string());
        }
        x = selector(x);
    } else if SC::symbol() == NUMBER {
        x = ST::Symbol {
            type_name: "Const".to_string(),
            val: SC::val_number(),
            tp: ST::PrimitiveTypes {
                type_name: "Int".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        x = CGwat::genConst(x);
        SC::getSym();
    } else if SC::symbol() == LPAREN {
        SC::getSym();
        x = expression();
        if SC::symbol() == RPAREN {
            SC::getSym();
        } else {
            SC::mark(") expected".to_string());
        }
    } else if SC::symbol() == NOT {
        SC::getSym();
        x = factor();
        if x.tp.type_name != "Bool" {
            SC::mark("not boolean".to_string());
        } else if x.type_name == "Const" {
            x.val = 1 - x.val;
        } else {
            x = CGwat::genUnaryOp(NOT, x);
        }
    } else {
        x = ST::Symbol {
            type_name: "None".to_string(),
            val: 0,
            ..Default::default()
        };
    }
    return x;
}

pub fn term() -> ST::Symbol {
    let mut x = factor();
    while SC::symbol() == TIMES || SC::symbol() == DIV || SC::symbol() == MOD || SC::symbol() == AND
    {
        let mut op = SC::symbol();
        SC::getSym();
        if op == AND && x.type_name != "Const" {
            x = CGwat::genUnaryOp(AND, x);
        }

        let mut y = factor();
        if x.tp.type_name == "Int"
            && y.tp.type_name == "Int"
            && (op == TIMES || op == DIV || op == MOD)
        {
            if x.type_name == "Const" && y.type_name == "Const" {
                if op == TIMES {
                    x.val = x.val * y.val;
                } else if op == DIV {
                    x.val = x.val / y.val;
                } else if op == MOD {
                    x.val = x.val % y.val;
                }
            } else {
                x = CGwat::genBinaryOp(op, x, y);
            }
        } else if x.tp.type_name == "Bool" && y.tp.type_name == "Bool" && op == AND {
            if x.type_name == "Const" {
                // x.val is suppoesed to be a boolean (assuming 1 == true)
                if x.val == 1 {
                    x = y;
                }
            } else {
                x = CGwat::genBinaryOp(AND, x, y);
            }
        } else {
            SC::mark("bad type! (1)".to_string());
        }
    }
    return x;
}

pub fn simpleExpression() -> ST::Symbol {
    let mut x = ST::Symbol {
        ..Default::default()
    };
    if SC::symbol() == PLUS {
        SC::getSym();
        x = term();
    } else if SC::symbol() == MINUS {
        SC::getSym();
        x = term();
        if x.tp.type_name != "Int" {
            SC::mark("Bad type!".to_string());
        } else if x.type_name == "Const" {
            x.val = -x.val;
        } else {
            x = CGwat::genUnaryOp(MINUS, x);
        }
    } else {
        x = term();
    }
    while SC::symbol() == PLUS || SC::symbol() == MINUS || SC::symbol() == OR {
        let mut op = SC::symbol();
        SC::getSym();
        if op == OR && x.type_name != "Const" {
            x = CGwat::genUnaryOp(OR, x);
        }
        let mut y = term();
        if x.tp.type_name == "Int" && y.tp.type_name == "Int" && (op == PLUS || op == MINUS) {
            if x.type_name == "Const" && y.type_name == "Const" {
                if op == PLUS {
                    x.val = x.val + y.val;
                } else if op == MINUS {
                    x.val = x.val - y.val;
                }
            } else {
                x = CGwat::genBinaryOp(op, x, y);
            }
        } else if x.tp.type_name == "Bool" && y.tp.type_name == "Bool" && op == OR {
            if x.type_name == "Const" {
                if x.val == 0 {
                    x = y;
                }
            } else {
                x = CGwat::genBinaryOp(OR, x, y);
            }
        } else {
            // println!("X type {} {}", x.tp.type_name, x.type_name);
            // println!("Y type {}", y.tp.type_name);
            SC::mark("bad type (2)".to_string());
        }
    }
    return x;
}

pub fn expression() -> ST::Symbol {
    let mut x = simpleExpression();
    while SC::symbol() == EQ
        || SC::symbol() == NE
        || SC::symbol() == LT
        || SC::symbol() == LE
        || SC::symbol() == GT
        || SC::symbol() == GE
    {
        let op = SC::symbol();
        SC::getSym();
        let y = simpleExpression();

        if (x.tp.type_name == "Int" && y.tp.type_name == "Int")
            || (x.tp.type_name == "Bool" && y.tp.type_name == "Bool")
        {
            if x.type_name == "Const" && y.type_name == "Const" {
                if op == EQ {
                    x.val = (x.val == y.val) as i32;
                } else if op == NE {
                    x.val = (x.val != y.val) as i32;
                } else if op == LT {
                    x.val = (x.val < y.val) as i32;
                } else if op == LE {
                    x.val = (x.val <= y.val) as i32;
                } else if op == GT {
                    x.val = (x.val > y.val) as i32;
                } else if op == GE {
                    x.val = (x.val >= y.val) as i32;
                }
                x = ST::Symbol {
                    val: x.val,
                    tp: ST::PrimitiveTypes {
                        type_name: "Bool".to_string(),
                        ..Default::default()
                    },
                    ..Default::default()
                };
            } else {
                x = CGwat::genRelation(op, x, y);
            }
        } else {
            SC::mark("Bad type".to_string());
        }
    }
    return x;
}

// allocVar == 1: use CGwat::genGlobalVars
// allocVar == 2: use CGwat::genGlobalLocalVars
pub fn declarations(allocVar: i32) -> i32 {
    // println!("in declarations() sym is: {}", SC::symbol());
    // println!("var sym is {}", VAR);
    if !FIRSTDECL(SC::symbol()) {
        if !FOLLOWDECL(SC::symbol()) {
            SC::mark("'begin' or declaration expected".to_string());
            while !FIRSTDECL(SC::symbol()) || !FOLLOWDECL(SC::symbol()) || !STRONGSYMS(SC::symbol())
            {
                SC::getSym();
            }
        }
    }
    while SC::symbol() == CONST {
        SC::getSym();
        if SC::symbol() == IDENT {
            let ident = SC::val_string();
            SC::getSym();
            if SC::symbol() == EQ {
                SC::getSym();
            } else {
                SC::mark("= expected".to_string());
            }
            let mut x = expression();
            if x.type_name == "Const" {
                ST::newDecl(ident, x);
            } else {
                SC::mark("expression not constant".to_string());
            }
        } else {
            SC::mark("constant name expected".to_string());
        }
        if SC::symbol() == SEMICOLON {
            SC::getSym();
        } else {
            SC::mark("; expected".to_string());
        }
    }
    while SC::symbol() == TYPE {
        SC::getSym();
        if SC::symbol() == IDENT {
            let ident = SC::val_string();
            if SC::symbol() == EQ {
                SC::getSym();
            } else {
                SC::mark("= expected".to_string());
            }
            let mut x = typ();
            ST::newDecl(ident, x);
            if SC::symbol() == SEMICOLON {
                SC::getSym();
            } else {
                SC::mark("; expected".to_string());
            }
        } else {
            SC::mark("type name expected".to_string());
        }
    }

    while SC::symbol() == VAR {
        SC::getSym();
        typedIds(ST::Symbol {
            type_name: "Var".to_string(),
            ..Default::default()
        });
        if SC::symbol() == SEMICOLON {
            SC::getSym();
        } else {
            SC::mark("; expected".to_string());
        }
    }
    let mut topScope = ST::topScope();
    let start = topScope.len() as i32;

    let mut varsize = 0;
    if allocVar == 1 {
        varsize = CGwat::genGlobalVars(&mut topScope, start);
    }
    return varsize;
}

pub fn compoundStatement() -> ST::Symbol {
    if SC::symbol() == BEGIN {
        SC::getSym();
    } else {
        SC::mark("begin expected".to_string());
    }
    let mut x = statement();
    while SC::symbol() == SEMICOLON || FIRSTSTATEMENT(SC::symbol()) {
        if SC::symbol() == SEMICOLON {
            SC::getSym()
        } else {
            SC::mark("; missing".to_string());
        }
        let mut y = statement();
        // x = CGwat::genSeq(x, y);
    }
    if SC::symbol() == END {
        SC::getSym()
    } else {
        SC::mark("'end' expected".to_string());
    }
    return x;
}

pub fn statement() -> ST::Symbol {
    let mut x = ST::Symbol {
        ..Default::default()
    };
    if !(FIRSTSTATEMENT(SC::symbol())) {
        SC::mark("'end' expected".to_string());
        SC::getSym();
        while !(FIRSTSTATEMENT(SC::symbol()))
            && !(FOLLOWSTATEMENT(SC::symbol()))
            && !(STRONGSYMS(SC::symbol()))
        {
            SC::getSym();
        }
    }
    if SC::symbol() == IDENT {
        x = ST::find(SC::val_string());
        SC::getSym();
        if x.type_name == "Var".to_string() || x.type_name == "Ref".to_string() {
            x = CGwat::genVar(x);
            x = selector(x);
            if SC::symbol() == BECOMES {
                SC::getSym();
                let mut y = expression();
                if x.tp.type_name.to_string() == y.tp.type_name.to_string()
                    && (x.tp.type_name.to_string() == "Bool".to_string()
                        || x.tp.type_name.to_string() == "Int".to_string())
                {
                    let throwawayX = x.clone();
                    CGwat::genAssign(throwawayX, y);
                } else {
                    println!(
                        "Incompatible assignment debug: x:{} y:{}",
                        x.tp.type_name, y.tp.type_name
                    );
                    SC::mark("incompatible assignment".to_string());
                }
            } else if SC::symbol() == EQ {
                SC::mark(":= expected".to_string());
                SC::getSym();
                let mut y = expression();
            } else {
                SC::mark(":= expected".to_string());
            }
        } else if x.type_name == "Proc".to_string() || x.type_name == "StdProc".to_string() {
            // let mut fp = x.par;
            // let mut ap: Vec<ST::Symbol>;
            // let mut i = 0;
            // if SC::symbol() == LPAREN {
            //     let mut y = expression();
            //     if i < fp.length() {
            //         if (fp[i].to_string() == "Var".to_string() || y.type_name.to_string() == "Var".to_string) && fp[i].to_string() == y.type_name.to_string() {
            //             if x.type_name.to_string() == "Proc".to_string()
            //         }
            //     }
            // }
            if x.type_name.to_string() == "StdProc".to_string() {
                let mut y = expression();
                if x.name.to_string() == "read".to_string() {
                    CGwat::genRead(y);
                } else if x.name.to_string() == "write".to_string() {
                    CGwat::genWrite(y);
                }  else if x.name.to_string() == "writeln".to_string() {
                    CGwat::genWriteln();
                } 
            }
        } else {
            SC::mark("variable or procedure expected".to_string());
        }
    } else if SC::symbol() == BEGIN {
        x = compoundStatement();
    } else if SC::symbol() == IF {
        SC::getSym();
        x = expression();
        if x.tp.type_name == "Bool".to_string() {
            x = CGwat::genThen(x);
        } else {
            SC::mark("boolean expected".to_string());
        }
        if SC::symbol() == THEN {
            SC::getSym();
        } else {
            SC::mark("then expected".to_string());
        }
        let mut y = statement();
        if SC::symbol() == ELSE {
            if x.tp.type_name == "Bool".to_string() {
                CGwat::genElse();
            }
            SC::getSym();
            let mut z = statement();
            if x.tp.type_name == "Bool".to_string() {
                CGwat::genIfElse();
            }
        } else {
            if x.tp.type_name == "Bool".to_string() {
                CGwat::genIfThen();
            }
        }
    } else if SC::symbol() == WHILE {
        SC::getSym();
        let mut t = CGwat::genWhile();
        x = expression();
        if x.tp.type_name == "Bool".to_string() {
            x = CGwat::genDo(x)
        } else {
            SC::mark("boolean expected".to_string());
        }
        if SC::symbol() == DO {
            SC::getSym();
        } else {
            SC::mark("do expected".to_string());
        }
        let mut y = statement();
        if x.tp.type_name == "Bool".to_string() {
            CGwat::genWhileDo();
        }
    } else {
        x = ST::Symbol {
            type_name: "None".to_string(),
            ..Default::default()
        };
    }
    return x;
}

pub fn typ() -> ST::Symbol {
    let mut x = ST::Symbol {
        ..Default::default()
    };
    if !(FIRSTTYPE(SC::symbol())) {
        SC::mark("type expected".to_string());
        while !(FIRSTTYPE(SC::symbol()))
            || !(FOLLOWTYPE(SC::symbol()))
            || !(STRONGSYMS(SC::symbol()))
        {
            SC::getSym();
        }
    }
    if SC::symbol() == IDENT {
        let ident = SC::val_string();
        x = ST::find(ident);
        SC::getSym();
        if x.type_name == "Type".to_string() {
            x = ST::Symbol {
                val: x.val,
                type_name: "Type".to_string(),
                ..Default::default()
            };
        } else {
            SC::mark("not a type".to_string());
        }
    } else if SC::symbol() == ARRAY {
        SC::getSym();
        if SC::symbol() == LBRAK {
            SC::getSym()
        } else {
            SC::mark("'[' expected".to_string());
        }
        let mut x = expression();
        if SC::symbol() == PERIOD {
            SC::getSym();
        } else {
            SC::mark("'.' expected".to_string());
        }
        if SC::symbol() == PERIOD {
            SC::getSym();
        } else {
            SC::mark("'.' expected".to_string());
        }
        let mut y = expression();
        if SC::symbol() == RBRAK {
            SC::getSym()
        } else {
            SC::mark("']' expected".to_string());
        }
        if SC::symbol() == OF {
            SC::getSym()
        } else {
            SC::mark("'of' expected".to_string());
        }
        let mut z = typ().val;
        if x.type_name != "Const".to_string() || x.val < 0 {
            SC::mark("bad lower bound".to_string());
            x = ST::Symbol {
                type_name: "None".to_string(),
                ..Default::default()
            };
        } else if y.type_name != "Const".to_string() || y.val < x.val {
            SC::mark("bad upper bound".to_string());
            x = ST::Symbol {
                type_name: "None".to_string(),
                ..Default::default()
            };
        } else {
            x = ST::Symbol {
                type_name: "Array".to_string(),
                base: ST::Base { size: z },
                lower: x.val,
                length: (y.val - x.val + 1),
                size: (x.val * z), 
                ..Default::default()
            }; 
        }
    } else if SC::symbol() == RECORD {
        SC::getSym();
        ST::openScope();
        typedIds(ST::Symbol {
            type_name: "Var".to_string(),
            ..Default::default()
        });
        while SC::symbol() == SEMICOLON {
            SC::getSym();
            typedIds(ST::Symbol {
                type_name: "Var".to_string(),
                ..Default::default()
            });
        }
        if SC::symbol() == END {
            SC::getSym();
        } else {
            SC::mark("'end' expected".to_string());
        }
        let mut r = ST::topScope();
        ST::closeScope();
        x = ST::Symbol {
            type_name: "Rec".to_string(),
            ..Default::default()
        };
    } else {
        x = ST::Symbol {
            type_name: "None".to_string(),
            ..Default::default()
        };
    }
    return x;
}

pub fn typedIds(kind: ST::Symbol) {
    let mut tid = vec![];
    if SC::symbol() == IDENT {
        tid.push(SC::val_string());
        SC::getSym();
    } else {
        SC::mark("identifier expected".to_string());
    }
    while SC::symbol() == COMMA {
        SC::getSym();
        if SC::symbol() == IDENT {
            tid.push(SC::val_string());
            SC::getSym();
        } else {
            SC::mark("identifier expected".to_string());
        }
    }
    if SC::symbol() == COLON {
        SC::getSym();
        let mut tp = typ().type_name;
        if tp != "None".to_string() {
            // prev line was typ().val and this line was typ.type_name but changed due to type error
            for i in 0..tid.len() {
                // println!("in typ: {}", SC::val_string());
                let mut s;
                if SC::val_string() == "boolean" {
                    s = "Bool";
                } else {
                    s = "Int";
                }
                ST::newDecl(
                    tid[i].to_string(),
                    ST::Symbol {
                        type_name: kind.type_name.to_string(),
                        tp: ST::PrimitiveTypes {
                            type_name: s.to_string(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                );
            }
        }
    } else {
        SC::mark("':' expected".to_string());
    }
}

pub fn program() -> String {
    ST::newDecl(
        "boolean".to_string(),
        CGwat::genBool(ST::Symbol {
            type_name: "Type".to_string(),
            tp: ST::PrimitiveTypes {
                type_name: "Bool".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }),
    );
    ST::newDecl(
        "integer".to_string(),
        CGwat::genInt(ST::Symbol {
            type_name: "Type".to_string(),
            tp: ST::PrimitiveTypes {
                type_name: "Int".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }),
    );
    ST::newDecl(
        "true".to_string(),
        ST::Symbol {
            type_name: "Const".to_string(),
            val: 1,
            tp: ST::PrimitiveTypes {
                type_name: "Bool".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
    );
    ST::newDecl(
        "false".to_string(),
        ST::Symbol {
            type_name: "Const".to_string(),
            val: 0,
            tp: ST::PrimitiveTypes {
                type_name: "Bool".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    // skip procs for now
    // newDecl('read', StdProc([Ref(Int)]))
    ST::newDecl("read".to_string(),
        ST::Symbol {type_name: "StdProc".to_string(), ..Default::default() });

    ST::newDecl("write".to_string(),
        ST::Symbol {type_name: "StdProc".to_string(), ..Default::default() });
    ST::newDecl("writeln".to_string(),
        ST::Symbol {type_name: "StdProc".to_string(), ..Default::default() });


    CGwat::genProgStart();
    // println!("in P0, sym is: {}", SC::symbol());
    if SC::symbol() == PROGRAM {
        SC::getSym();
    } else {
        SC::mark("'program' expected".to_string());
    }
    if SC::symbol() == IDENT {
        SC::getSym();
    } else {
        SC::mark("program name expected".to_string());
    }
    if SC::symbol() == SEMICOLON {
        SC::getSym();
    } else {
        SC::mark("; expected".to_string());
    }
    declarations(1);
    CGwat::genProgEntry(0);
    let x = compoundStatement();
    return CGwat::genProgExit();
}

// since we're only compiling for CGwat there's no need to target/dstfn
pub fn compileString(src: String, testCase: String) {
    SC::init(src);
    ST::init();
    let p = program();

    // prints the wat generated code
    println!("\n--------------------------------------------------------");
    println!("-------- TEST CASE: {}", testCase);
    println!("--------------------------------------------------------");
    println!("{}", p);
    CGwat::asm_clear();
}

// writes to a file called srcfn
pub fn compileFile(srcfn: String) {}