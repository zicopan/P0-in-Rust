/*
P0 test file

use P0test::testAll() in  main to run all tests
*/

use crate::P0;
use crate::ST;

pub fn testAll() {
    
    P0::compileString(
        "program p;\n
        var x, y, z: integer;\n
        begin\n
        y := 17;\n
        x := 2;\n
        z := x + y\n
        end"
        .to_string(), "variable addition".to_string()
    );

    P0::compileString(
        "program p;\n
        var x, y, z: integer;\n
        begin\n
        x := 12;\n
        y := 6;\n
        z := x div y\n
        end"
        .to_string(), "variable division".to_string()
    );
    
    P0::compileString(
        "program p;\n
        var x: integer;\n
        begin\n
        x := 0;\n
        while x < 5 do x := x + 1\n
        end"
        .to_string(), "loop with '<'".to_string()
    );

    P0::compileString(
        "program p;\n
        var x: integer;\n
        begin\n
        x := 8;\n
        while x > 5 do x := x - 1\n
        end"
        .to_string(), "loop with '>'".to_string()
    );

    // Sample A
    P0::compileString(
        "program p;\n
        var x: integer;\n
        begin\n
        x := 2;\n
        while (x <= 5) do x := x + 1\n
        end"
        .to_string(), "loop with '<='".to_string()
    );

    P0::compileString(
        "program p;\n
        var x: integer;\n
        begin\n
        x := 8;\n
        if x = 8 then x := 2 else x := 3
        end"
        .to_string(), "if-else statement".to_string()
    );

    P0::compileString(
        "program p;\n
        var b, t, f: boolean;\n
        begin\n
            b := true;\n
            if b then f := false;
          writeln();\n
          writeln()
        end"
        .to_string(), "bool assign and if-boolean statement".to_string()
    );

    // Sample C
    P0::compileString(
        "program p;\n
        var x, y, z: integer;\n
        begin\n
        x := 3;\n
        if x < 10 then while x < 10 do x := x + 1;\n
        y := x + 0;\n
        y := x * 1;\n
        x := 1 * y;\n
        y := x div 1;\n
        y := 40;\n
        x := 3;\n
        while y > 20 do while x = 3 do y := y - 1;
        x := y - 0;\n
        z := x + y\n
        end"
        .to_string(), "long maths with while within while loop".to_string()
    );

    P0::compileString(
        "program p;\n
        const b = true;\n
        begin\n
         if b then writeln();\n
          writeln()\n
        end"
        .to_string(), "const boolean assign".to_string()
    );

    // Sample D
    P0::compileString(
        "program p;
        var x: integer;
        begin read(x);
          x := 3 * x;
          write(x);
          writeln();
          writeln();
          write(x * 5)
        end".to_string(), "write test".to_string()
    );

    // Sample E
    P0::compileString(
        "program p;
        const five = 5;
        const seven = 7;
        const always = true;
        const never = false;
        var x, y, z: integer;
        var b, t, f: boolean;
        begin 
        x := seven; 
        y := 9; z := 11; 
        t := true; 
        f := false;
          if true then write(7) else write(9);    
          if false then write(7) else write(9);   
          if t then write(7) else write(9);       
          if f then write(7) else write(9);       
          if not t then write(7) else write(9);   
          if not f then write(7) else write(9);   
          if t or t then write(7) else write(9);  
          if t or f then write(7) else write(9);  
          if f or t then write(7) else write(9);  
          if f or f then write(7) else write(9);  
          if t and t then write(7) else write(9); 
          if t and f then write(7) else write(9); 
          if f and t then write(7) else write(9); 
          if f and f then write(7) else write(9); 
          writeln();
          b := true;
          if b then write(3) else write(5); 
          b := false;
          if b then write(3) else write(5); 
          b := x < y;
          if b then write(x) else write(y); 
          b := (x > y) or t;
          if b then write(3) else write(5); 
          b := (x > y) or f;
          if b then write(3) else write(5); 
          b := (x = y) or (x > y);
          if b then write(3) else write(5); 
          b := (x = y) or (x < y);
          if b then write(3) else write(5); 
          b := f and (x >= y);
          if b then write(3) else write(5); 
          writeln();
          while y > 3 do                    
            begin write(y); y := y - 1 end;
          write(y); writeln();              
          if not(x < y) and t then          
            write(x)
        end".to_string(), "long conditionals and writes (group 11)".to_string()
    );
    
    P0::compileString("
        program  p;
        const two  = 2;
        var x, y: integer;
        begin
            x := 2;
            x := x + 3;
            if (x < two) then x := 1 else x := 10;
            y := x * 3;
            write(x);
        end
        ".to_string(), "demo test case".to_string());



































    // big test case for LARGE LOC:
    // P0::compileString(
    //     "program p;
    //     const one = 1;
    //     const two = 2;
    //     const five = 5;
    //     const six = 6;
    //     const seven = 7;
    //     const yes = true;
    //     const no = false;
    //     var x: integer;
    //     var y: integer;
    //     var z: integer;
    //     var b: boolean; 
    //     var t: boolean; 
    //     var f: boolean;
    //     begin 
    //         z := 11;  
    //         t := false; 
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z - y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z div y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z * y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z + y;
    //         z := 11;  
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11; 
    //         x := z - y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z div y;
    //         z := 11;  
    //         z := 11;  
    //         x := z div y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z * y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z + y;
    //         z := 11;  
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z * y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z + y;
    //         z := 11;  
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();              
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z - y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z div y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z * y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z + y;
    //         z := 11;  
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();            
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z - y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z div y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z * y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z + y;
    //         z := 11;  
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();              
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z - y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z div y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z * y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z + y;
    //         z := 11;  
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();              
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z - y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z div y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z * y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z + y;
    //         z := 11;  
    //         z := 11;  
        //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();            
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z - y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z div y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z * y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z + y;
    //         z := 11;  
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();              
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z - y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z div y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z * y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z + y;
    //         z := 11;  
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();              
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z - y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z div y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z * y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
        //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();            
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z - y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z div y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z * y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z + y;
    //         z := 11;  
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();              
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z - y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z div y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z * y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z + y;
    //         z := 11;  
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();              
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z - y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z div y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z * y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y;
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         z := 11;  
    //         z := 11;  
    //         x := seven; 
    //         writeln();  
    //         z := 11; 
    //         x := z mod y
    //     end"
    //     .to_string()
    // );
}