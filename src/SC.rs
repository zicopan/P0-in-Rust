/*
Implementation of the Scanner
*/
// #[allow(dead_code)]
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct ValType {
    pub number: i32,
    pub string: String,

    // none = 1 means type is null
    pub none: i32,
    pub type_name: String,
}

pub struct SourceType {
    pub src: String,
}

static TIMES: i32 = 1;
static DIV: i32 = 2;
static MOD: i32 = 3;
static AND: i32 = 4;
static PLUS: i32 = 5;
static MINUS: i32 = 6;
static OR: i32 = 7;
static EQ: i32 = 8;
static NE: i32 = 9;
static LT: i32 = 10;
static GT: i32 = 11;
static LE: i32 = 12;
static GE: i32 = 13;
static PERIOD: i32 = 14;
static COMMA: i32 = 15;
static COLON: i32 = 16;
static RPAREN: i32 = 17;
static RBRAK: i32 = 18;
static OF: i32 = 19;
static THEN: i32 = 20;
static DO: i32 = 21;
static LPAREN: i32 = 22;
static LBRAK: i32 = 23;
static NOT: i32 = 24;
static BECOMES: i32 = 25;
static NUMBER: i32 = 26;
static IDENT: i32 = 27;
static SEMICOLON: i32 = 28;
static END: i32 = 29;
static ELSE: i32 = 30;
static IF: i32 = 31;
static WHILE: i32 = 32;
static ARRAY: i32 = 33;
static RECORD: i32 = 34;
static CONST: i32 = 35;
static TYPE: i32 = 36;
static VAR: i32 = 37;
static PROCEDURE: i32 = 38;
static BEGIN: i32 = 39;
static PROGRAM: i32 = 40;
static EOF: i32 = 41;

static mut line: i32 = 0;
static mut lastline: i32 = 0;
static mut errline: i32 = 0;
static mut pos: i32 = 0;
static mut lastpos: i32 = 0;
static mut errpos: i32 = 0;
static mut sym: Option<i32> = Some(0);
static mut error: bool = false;
static mut index: i32 = 0;
static mut ch: char = '\0';
static CHAR_0: char = '\0';

lazy_static! {
    static ref source: Mutex<String> = Mutex::new(String::new());
    static ref val: Mutex<ValType> = Mutex::new(ValType {
        number: 0 as i32,
        string: "".to_string(),
        none: 1 as i32,
        type_name: "none".to_string(),
    });
}

pub fn init(src: String) {
    unsafe {
        line = 1;
        lastline = 1;
        errline = 1;

        pos = 0;
        lastpos = 0;
        errpos = 0;
        error = false;
        *source.lock().unwrap() = src;
        index = 0;

        getChar();
        getSym();
    }
}

pub fn getChar() {
    unsafe {
        if index == source.lock().unwrap().len() as i32 {
            ch = '\0';
        } else {
            ch = source.lock().unwrap().chars().nth(index as usize).unwrap();
            index = index + 1;
            lastpos = pos;

            if ch == '\n' {
                pos = 0;
                line = line + 1;
            } else {
                lastline = line;
                pos = pos + 1;
            }
        }
    }
}

pub fn mark(msg: String) {
    unsafe {
        if lastline > errline || lastpos > errpos {
            println!(
                "error: {}', {}, {}, {}, {}",
                line, lastline, pos, lastpos, msg
            );
        }
        errline = lastline;
        errpos = lastpos;
        error = true;
    }
}

pub fn number() {
    unsafe {
        sym = Some(NUMBER);
        val.lock().unwrap().number = 0;

        pub fn chToInt(c: char) -> i32 {
            return match c {
                '0' => 0 as i32,
                '1' => 1 as i32,
                '2' => 2 as i32,
                '3' => 3 as i32,
                '4' => 4 as i32,
                '5' => 5 as i32,
                '6' => 6 as i32,
                '7' => 7 as i32,
                '8' => 8 as i32,
                '9' => 9 as i32,
                _ => -1 as i32,
            };
        }

        while '0' <= ch && ch <= '9' {
            let tempVal = val.lock().unwrap().number.clone();
            val.lock().unwrap().number = 10 * tempVal + chToInt(ch);
            getChar();
        }

        if val.lock().unwrap().number >= 2_147_483_647i32 {
            mark("number too large".to_string());
            val.lock().unwrap().number = 0;
        }
    }
}

pub fn keywordLookup(s: String) -> i32 {
    return match s.as_ref() {
        "div" => DIV,
        "mod" => MOD,
        "and" => AND,
        "or" => OR,
        "of" => OF,
        "then" => THEN,
        "do" => DO,
        "not" => NOT,
        "end" => END,
        "else" => ELSE,
        "if" => IF,
        "while" => WHILE,
        "array" => ARRAY,
        "record" => RECORD,
        "const" => CONST,
        "type" => TYPE,
        "var" => VAR,
        "procedure" => PROCEDURE,
        "begin" => BEGIN,
        "program" => PROGRAM,
        _ => -1,
    };
}

pub fn identKW() {
    unsafe {
        let mut KEYWORDS = HashMap::new();
        KEYWORDS.insert("div".to_string(), DIV);
        KEYWORDS.insert("mod".to_string(), MOD);
        KEYWORDS.insert("and".to_string(), AND);
        KEYWORDS.insert("or".to_string(), OR);
        KEYWORDS.insert("of".to_string(), OF);
        KEYWORDS.insert("then".to_string(), THEN);
        KEYWORDS.insert("do".to_string(), DO);
        KEYWORDS.insert("not".to_string(), NOT);
        KEYWORDS.insert("end".to_string(), END);
        KEYWORDS.insert("else".to_string(), ELSE);
        KEYWORDS.insert("if".to_string(), IF);
        KEYWORDS.insert("while".to_string(), WHILE);
        KEYWORDS.insert("array".to_string(), ARRAY);
        KEYWORDS.insert("record".to_string(), RECORD);
        KEYWORDS.insert("const".to_string(), CONST);
        KEYWORDS.insert("type".to_string(), TYPE);
        KEYWORDS.insert("var".to_string(), VAR);
        KEYWORDS.insert("procedure".to_string(), PROCEDURE);
        KEYWORDS.insert("begin".to_string(), BEGIN);
        KEYWORDS.insert("program".to_string(), PROGRAM);
        let start = index - 1;
        while ('A' <= ch && ch <= 'Z') || ('a' <= ch && ch <= 'z') || ('0' <= ch && ch <= '9') {
            getChar();
        }

        let sourceStr: String = source
            .lock()
            .unwrap()
            .chars()
            .skip(start as usize)
            .take((index - start) as usize)
            .collect();

        val.lock().unwrap().string = source
            .lock()
            .unwrap()
            .chars()
            .skip(start as usize)
            .take((index - start - 1) as usize)
            .collect();

        if keywordLookup(val.lock().unwrap().string.to_string()) != -1 {
            // println!("Matched keyword: {}", val.lock().unwrap().string);
            sym = Some(*KEYWORDS.get(&val.lock().unwrap().string).unwrap());
        // Need this else if as the end keyword may not have whitespace after
        } else if sourceStr == "end" {
            val.lock().unwrap().string = source
                .lock()
                .unwrap()
                .chars()
                .skip(start as usize)
                .take((index - start) as usize)
                .collect();
            sym = Some(*KEYWORDS.get(&val.lock().unwrap().string).unwrap());
        } else {
            // println!(
            //     "Unmatched keyword: {}",
            //     val.lock().unwrap().string.to_string()
            // );
            sym = Some(IDENT);
        }
    }
}

pub fn comment() {
    unsafe {
        while ch != CHAR_0 && ch != '}' {
            getChar();
        }

        if ch == CHAR_0 {
            mark("comment not terminated".to_string());
        } else {
            getChar();
        }
    }
}

pub fn symbol() -> i32 {
    return unsafe { sym.unwrap_or(SEMICOLON) };
}

pub fn val_number() -> i32 {
    return unsafe { val.lock().unwrap().number };
}

pub fn val_string() -> String {
    return val.lock().unwrap().string.clone();
}

pub fn getSym() {
    unsafe {
        while CHAR_0 < ch && ch <= ' ' {
            getChar();
        }
        if ('A' <= ch && ch <= 'Z') || ('a' <= ch && ch <= 'z') {
            // println!("getSym parsed: {}", ch);
            identKW();
        } else if '0' <= ch && ch <= '9' {
            number();
        } else {
            match ch {
                '{' => {
                    comment();
                    getSym();
                }
                '*' => {
                    getChar();
                    sym = Some(TIMES);
                }
                '+' => {
                    getChar();
                    sym = Some(PLUS);
                }
                '-' => {
                    getChar();
                    sym = Some(MINUS);
                }
                '=' => {
                    getChar();
                    sym = Some(EQ);
                }
                '<' => {
                    getChar();
                    if ch == '=' {
                        getChar();
                        sym = Some(LE)
                    } else if ch == '>' {
                        getChar();
                        sym = Some(NE);
                    } else {
                        sym = Some(LT);
                    }
                }
                '>' => {
                    getChar();
                    if ch == '=' {
                        getChar();
                        sym = Some(GE);
                    } else {
                        sym = Some(GT);
                    }
                }
                ';' => {
                    getChar();
                    sym = Some(SEMICOLON);
                }
                ',' => {
                    getChar();
                    sym = Some(COMMA);
                }
                ':' => {
                    getChar();
                    if ch == '=' {
                        getChar();
                        sym = Some(BECOMES);
                    } else {
                        sym = Some(COLON);
                    }
                }
                '.' => {
                    getChar();
                    sym = Some(PERIOD);
                }
                '(' => {
                    getChar();
                    sym = Some(LPAREN);
                }
                ')' => {
                    getChar();
                    sym = Some(RPAREN);
                }
                '[' => {
                    getChar();
                    sym = Some(LBRAK);
                }
                ']' => {
                    getChar();
                    sym = Some(RBRAK);
                }
                '\0' => {
                    sym = Some(EOF);
                }
                _ => {
                    mark("illegal character".to_string());
                    getChar();
                    sym = None;
                }
            }
        }
    }
}
