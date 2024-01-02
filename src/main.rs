use std::env::args;
mod lib;
use lib::*;

const USAGE_DOC: &str = "\
Usage: bin2const <input_file> <output_const_name> <conversion_type> [tab_size] [output_file]
    <input_file>        The file to convert.
    <output_const_name> The name of the constant to generate. Has no effect if the conversion type
                        is bin or hex.
    <conversion_type>   The type of conversion to use. Can be bin, c, rust, csharp, python, javascript.
                        as well as most of their aliases.
    [tab_size]          The size of a tabulation in the output file. Per default is 4.
    [output_file]       Optional output file, if not specified, the output will be printed to stdout.
";

fn main() {
    let args = args().collect::<Vec<String>>();

    if args.len() < 4 {
        println!("{}", USAGE_DOC);
        return;
    }

    let input_file = args[1].clone();
    let output_const_name = args[2].clone();
    let conversion_type = args[3].clone();
    let tab_size = if args.len() > 4 {
        args[4].parse::<usize>().unwrap_or(4)
    } else {
        4
    };
    let output_file = if args.len() > 5 {
        Some(args[5].clone())
    } else {
        None
    };

    let binary = match file_to_binary(&input_file) {
        Ok(binary) => binary,
        Err(e) => {
            println!("Error while reading file: {}", e);
            return;
        }
    };

    let out = match conversion_type.to_ascii_lowercase().trim() {
        "bin" | "binary" | "raw" => {
            binary_to_binary(&binary)
        }
        "hex" | "hexadecimal" | "hexa" | "hexa-decimal" | "hexa_decimal" => binary_to_hex(&binary),
        "c" | "cpp" | "c++" | "cxx" | "h" | "hpp" | "h++" | "hxx"  => {
            binary_to_c_const(&binary, &output_const_name, tab_size)
        }
        "cdef" | "c-def" | "c_def" | "def" | "define" | "cppdef"  => {
            binary_to_c_define(&binary, &output_const_name, tab_size)
        }
        "rust" | "rs" | "rustlang" | "rust-lang" => {
            binary_to_rust_const(&binary, &output_const_name, tab_size)
        }
        "csharp" | "cs" | "c#" | "c-sharp" | "c_sharp" => {
            binary_to_csharp_const(&binary, &output_const_name, tab_size)
        }
        "python" | "py" | "python3" | "py3" | "python_3" => {
            binary_to_python_const(&binary, &output_const_name, tab_size)
        }
        "javascript" | "js" | "typescript" | "ts" => {
            binary_to_javascript_const(&binary, &output_const_name, tab_size)
        }
        _ => {
            println!("Unknown conversion type: {}", conversion_type);
            return;
        }
    };

    match output_file {
        Some(output_file) => match std::fs::write(output_file, out) {
            Ok(_) => (),
            Err(e) => {
                println!("Error while writing to file: {}", e);
                return;
            }
        },
        None => println!("{}", out),
    }
}
