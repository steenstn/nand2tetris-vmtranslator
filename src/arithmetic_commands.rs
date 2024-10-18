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
