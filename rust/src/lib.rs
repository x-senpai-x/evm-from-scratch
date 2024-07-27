use primitive_types::U256;
use std::env::args;//we want to use command line arguments

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    let mut pc = 0;

    let code = _code.as_ref();

    while pc +1 < code.len() {
        let opcode = code[pc];//Fetches the current instriction
        pc += 1;//increaments the program counter to print the next instruction

        if opcode == 0x00 {
            //STOP
            break;

        }
        else if opcode== 0x5F{
            let zero_u256: U256 = U256::from(0);
            stack.push(zero_u256);

        }
        
        

    }

    // TODO: Implement me

    return EvmResult {
        stack: stack,
        success: true,
    };
}
