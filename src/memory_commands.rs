use indoc::formatdoc;

fn get_hack_name(segment: &str) -> String {
    let res = match segment {
        "local" => "LCL",
        "argument" => "ARG",
        "this" => "THIS",
        "that" => "THAT",
        "temp" => "5",
        _ => panic!("Unknown segment: {}", segment),
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

pub fn pop(segment: &str, value: i32, filename: String) -> String {
    match segment {
        "pointer" => pop_pointer(value),
        "static" => pop_static(value, filename),
        "temp" => pop_temp(value),
        _ => pop_segment(segment, value),
    }
}

pub fn push_constant(value: i32) -> String {
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
    let address = get_this_that_address(value);
    formatdoc!(
        "// push pointer {address}
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

fn pop_pointer(value: i32) -> String {
    let address = get_this_that_address(value);

    formatdoc!(
        "// pop pointer {address}
@SP
M=M-1
A=M
D=M
@{address}
M=D
"
    )
}

fn get_this_that_address(value: i32) -> String {
    match value {
        0 => "THIS".to_string(),
        1 => "THAT".to_string(),
        _ => panic!("Unexpected pointer value: {}", value),
    }
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

fn pop_static(value: i32, filename: String) -> String {
    formatdoc!(
        "// pop static {value}
@SP
M=M-1
A=M
D=M
@{filename}.{value}
M=D
"
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

fn pop_temp(value: i32) -> String {
    let result = value + 5;
    formatdoc!(
        "// pop temp $value
@SP
M=M-1
A=M
D=M
@{result}
M=D
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

fn pop_segment(segment: &str, value: i32) -> String {
    // pop local i => add = LCL + i; SP--; *addr = *SP
    let address = get_hack_name(segment);
    formatdoc!(
        "// pop {segment} {value}
// addr = segment + i
@{value}
D=A
@{address}
D=D+M
@R13
M=D

// SP--
@SP
M=M-1

// *addr = *SP
A=M
D=M
@R13
A=M
M=D
"
    )
}
