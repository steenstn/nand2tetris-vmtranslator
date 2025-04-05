use crate::memory_commands::push_constant;
use indoc::formatdoc;

pub fn label(label: &str, function_name: &str) -> String {
    formatdoc!(
        "({}.{})
",
        function_name,
        label
    )
}

pub fn goto(label: &str, current_function_name: &str) -> String {
    formatdoc!(
        "// goto
@{}.{}
0;JMP
",
        current_function_name,
        label
    )
}

/* TODO
When

https://drive.google.com/file/d/1BexrNmdqYhKPkqD_Y81qNAUeyfzl-ZtO/view sida 81

*/
pub fn if_goto(label: &str, current_function_name: &str) -> String {
    formatdoc!(
        "// if-goto
@SP
M=M-1
A=M
D=M
@{}.{}
D;JNE
",
        current_function_name,
        label
    )
}

pub fn return_asm() -> String {
    // return

    /*
    endFrame = LCL // endFrame is a temporary variable
    retAddr = *(endFrame - 5) // get the return address (the content)
    *ARG = pop() // Repositions the return value for the caller
    SP = ARG + 1 // Repositions SP of the caller
    THAT = *(endFrame - 1)
    THIS = *(endFrame - 2)
    ARG = *(endFrame - 3)
    LCL = *(endFrame - 4)
    goto retAddr
    */
    // R13 = endFrame
    // R14 = retAddr
    formatdoc!(
        "// return
// endFrame = LCL
@LCL
D=M
@R13
M=D
//retAddr = *(endFrame - 5)
@5
D=A
@R13
D=M-D
@R14
M=D
//*ARG = pop()
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
// SP = ARG + 1
@ARG
D=M+1
@SP
M=D
// THAT = *(endFrame - 1)
@1
D=A
@R13
D=M-D
A=D
D=M
@THAT
M=D
// THIS = *(endFrame -2)
@2
D=A
@R13
D=M-D
A=D
D=M
@THIS
M=D
// ARG = *(endFrame - 3)
@3
D=A
@R13
D=M-D
A=D
D=M
@ARG
M=D
// LCL = *(endFrame - 4)
@4
D=A
@R13
D=M-D
A=D
D=M
@LCL
M=D
// goto retAddr
@R14
A=M
0;JMP
",
    )
}

pub fn function_asm(function_name: &str, num_args: &str) -> String {
    // function SimpleFunction.test 2
    /*
    (functionName) // declare a label
    repeat nVars times:
      push 0
    * */
    let num_args_int = match i32::from_str_radix(num_args, 10) {
        Ok(res) => res,
        Err(why) => panic!("Failed to parse {} {} {}", function_name, num_args, why),
    };

    let mut result_string = "".to_owned();
    for _ in 0..num_args_int {
        result_string.push_str(push_constant(0).as_str());
    }

    formatdoc!(
        "// function {function_name} {num_args}
({function_name})
{result_string}
"
    )
}

pub fn call(
    function_name: &str,
    num_arguments: &str,
    file_name: &str,
    function_call_counter: &mut i32,
) -> String {
    // call Bar.mult 2
    /*

        we know how many arguments have been pushed

        push returnAddress // using the label declared below
        push LCL // saves LCL of the caller
        push ARG // saves ARG of the caller
        push THIS
        push THAT
        ARG = SP - 5 - nArgs
        LCL = SP
        goto functionName
        (return address) // declare label for the return address
    */
    let result = formatdoc!(
        "// call {function_name}
// Push return address
@{return_address}
D=A
@SP
A=M
M=D
@SP
M=M+1
// Push local
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
// Push ARG
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
// Push THIS
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// Push THAT
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
// ARG = SP - 5 - nArgs ({num_arguments})
@SP
D=M
@5
D=D-A
@{num_arguments}
D=D-A
@ARG
M=D
// LCL = SP
@SP
D=M
@LCL
M=D
// goto
@{function_name}
0;JMP
({return_address})
",
        return_address = format!("{}$ret.{}", function_name, function_call_counter)
    );
    *function_call_counter += 1;
    return result;
    // return address of outer method?
}
