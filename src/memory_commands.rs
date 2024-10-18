use indoc::formatdoc;

fn get_hack_name(segment: &str) -> String {
    let res = match segment {
        "local" => "LCL",
        "argument" => "ARG",
        "this" => "THIS",
        "that" => "THAT",
        "temp" => "5",
        _ => panic!("AAH {}", segment),
    };
    res.to_string()
}

pub fn push(segment: &str, value: i32, filename: String) -> String {
    match segment {
        "constant" => push_constant(value),
        "pointer" => push_pointer(value),
        "static" => push_static(value, filename),
        "temp" => push_temp(value),
        _ => push_segment(segment, value),
    }
}

fn push_constant(value: i32) -> String {
    formatdoc!(
        "// push constant {value}
        @{value}
        D=A
        @SP
        A=M
        M=D
        @SP
        M=M+1
"
    )
}

fn push_pointer(value: i32) -> String {
    let address = match value {
        0 => "THIS".to_string(),
        1 => "THAT".to_string(),
        _ => panic!("Unexpected pointer value {}", value),
    };
    formatdoc!(
        "// push pointer {address}
@{address}
@D=M
@SP
A=M
M=D
@SP
M=M+1
"
    )
}

fn push_static(value: i32, filename: String) -> String {
    formatdoc!(
        "// push static {value}
@{filename}.{value}
D=M
@SP
A=M
M=D
@SP
M=M+1
",
    )
}

fn push_temp(value: i32) -> String {
    let address = 5 + value;
    formatdoc!(
        "// push temp {value}
@{address}
D=M
@SP
A=M
M=D
@SP
M=M+1
"
    )
}

fn push_segment(segment: &str, value: i32) -> String {
    let address = get_hack_name(segment);
    formatdoc!(
        "// push {segment} {value}
        @{value}
D=A
@{address}
A=M+D
D=M
@SP
A=M
M=D
@SP
M=M+1
"
    )
}
