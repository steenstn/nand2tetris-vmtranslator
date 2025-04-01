use crate::memory_commands::push_constant;
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

/* TODO
When

https://drive.google.com/file/d/1BexrNmdqYhKPkqD_Y81qNAUeyfzl-ZtO/view sida 81

*/
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

pub fn return_asm() -> String {
    // return

    /*
    endFrame = LCL // endFrame is a temporary variable
    retAddr = *(endFrame - 5) // get the return address
    *ARG = pop() // Repositions the return value for the caller
    SP = ARG + 1 // Repositions SP of the caller
    THAT = *(endFrame - 1)
    THIS = *(endFrame - 2)
    ARG = *(endFrame - 3)
    LCL = *(endFrame - 4)
    goto retAddr
    */

    formatdoc!(
        "// return
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

pub fn call(function_name: &str, file_name: &str, function_call_counter: &mut i32) -> String {
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

goto {function_name}
({return_address})",
        return_address = format!(
            "{}.{}$ret.{}",
            file_name, function_name, function_call_counter
        )
    );
    *function_call_counter += 1;
    return result;
    // return address of outer method?
}
