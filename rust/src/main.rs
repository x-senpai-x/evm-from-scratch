/**
 * EVM From Scratch
 * Rust template
 *
 * To work on EVM From Scratch in Rust:
 *
 * - Install Rust: https://www.rust-lang.org/tools/install
 * - Edit `rust/lib.rs`
 * - Run `cd rust && cargo run` to run the tests
 *
 * Hint: most people who were trying to learn Rust and EVM at the same
 * gave up and switched to JavaScript, Python, or Go. If you are new
 * to Rust, implement EVM in another programming language first.
 */

use evm::evm;
use primitive_types::U256;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Evmtest {
    name: String,
    hint: String,
    code: Code,
    expect: Expect,
}

#[derive(Debug, Deserialize)]
struct Code {
    asm: String,
    bin: String,
}

#[derive(Debug, Deserialize)]
struct Expect {
    stack: Option<Vec<String>>,
    success: bool,
    // #[serde(rename = "return")]
    // ret: Option<String>,
}


fn main() {

    let text = std::fs::read_to_string("../evm.json").unwrap();
    // reads the json file into strings 

    let data: Vec<Evmtest> = serde_json::from_str(&text).unwrap();
    //serde_json deserealizes json string to text variable and then stored into vector where each element is of type struct Evmtest

    let total = data.len();
    //total should be equal to total  tests 
    //each test is a single element stored in vector as a struct 

    for (index, test) in data.iter().enumerate() {
        //data.iter() allows iterating over the vector data
        //enumerate adds index to each element
        //so tuple (index, test) is created for each element in vector data 
        //test is of type struct Evmtest
        println!("Test {} of {}: {}", index + 1, total, test.name);

        let code: Vec<u8> = hex::decode(&test.code.bin).unwrap();
        //decodes the hex string (&str) into bytes and stores in code variable
        //code is a vector of u8

        let result = evm(&code);
        //calls the evm function with argument as the bytecode


        let mut expected_stack: Vec<U256> = Vec::new();
        //stack holds U256 values
    

        //the below line checks if test.expect.stack holds a value or not

        //if it does it is assigned to staacks then it pushes the value into expected_stack
        // If test.expect.stack is None, the code inside the block will be skipped.
        if let Some(ref stacks) = test.expect.stack {
            for value in stacks {
                expected_stack.push(U256::from_str_radix(value, 16).unwrap());
                //str is converted to u256 and pushed into expected_stack
                //base 16 converted to u256

                //eg if test.expect.stack = ["0x01", "0x02", "0x03"]
                //then expected_stack = [1, 2, 3]

            }
        }

        let mut matching = result.stack.len() == expected_stack.len();
        if matching {
            for i in 0..result.stack.len() {
                if result.stack[i] != expected_stack[i] {
                    matching = false;
                    break;
                }
            }
        }
        
        matching = matching && result.success == test.expect.success;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");
            
            println!("Actual success: {:?}", result.success);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            panic!("Test failed");
        }
        println!("PASS");
    }
    println!("Congratulations!");
}
