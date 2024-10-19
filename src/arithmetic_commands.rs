use indoc::formatdoc;

pub fn add() -> String {
    formatdoc!(
        "// Add
    @SP
    M=M-1
    A=M
    D=M
    @SP
    M=M-1
    A=M
    M=M+D
    @SP
    M=M+1
"
    )
}

pub fn and() -> String {
    formatdoc!(
        "// And
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D&M
@SP
M=M+1
"
    )
}

pub fn neg() -> String {
    formatdoc!(
        "// neg
@SP
M=M-1
A=M
M=-M
@SP
M=M+1
"
    )
}

pub fn not() -> String {
    formatdoc!(
        "// not
@SP
M=M-1
A=M
M=!M
@SP
M=M+1
"
    )
}

pub fn or() -> String {
    formatdoc!(
        "// Or
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D|M
@SP
M=M+1
"
    )
}

pub fn sub() -> String {
    formatdoc!(
        "
// Sub
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1
"
    )
}

pub fn eq(branch_counter: &mut i32) -> String {
    let result = formatdoc!(
        "// Equals
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D-M
@EQUAL{branch_counter}
D;JEQ
@SP
A=M
M=0
@EQCONT{branch_counter}
0;JMP
(EQUAL{branch_counter})
@SP
A=M
M=-1
(EQCONT{branch_counter})
@SP
M=M+1
",
    );

    *branch_counter += 1;
    return result;
}

pub fn gt(branch_counter: &mut i32) -> String {
    let result = formatdoc!(
        "// gt
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@GREATERTHAN{branch_counter}
D;JGT
@SP
A=M
M=0
@GTCONT{branch_counter}
0;JMP
(GREATERTHAN{branch_counter})
@SP
A=M
M=-1
(GTCONT{branch_counter})
@SP
M=M+1
",
    );
    *branch_counter += 1;
    return result;
}

pub fn lt(branch_counter: &mut i32) -> String {
    let result = formatdoc!(
        "// lt
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@LESSTHAN{branch_counter}
D;JLT
@SP
A=M
M=0
@LTCONT{branch_counter}
0;JMP
(LESSTHAN{branch_counter})
@SP
A=M
M=-1
(LTCONT{branch_counter})
@SP
M=M+1
"
    );
    *branch_counter += 1;
    return result;
}
