use indoc::formatdoc;
pub fn label(label: &str) -> String {
    formatdoc!(
        "({label})
"
    )
}

pub fn goto(label: &str) -> String {
    formatdoc!(
        "// goto
@{label}
0;JMP
"
    )
}

pub fn if_goto(label: &str) -> String {
    formatdoc!(
        "// if-goto
@SP
M=M-1
A=M
D=M
@{label}
D;JNE
"
    )
}
