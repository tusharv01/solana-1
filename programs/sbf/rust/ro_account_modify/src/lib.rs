use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    pubkey::Pubkey,
};

const ARGUMENT_INDEX: usize = 0;

const INSTRUCTION_MODIFY: u8 = 0;
const INSTRUCTION_INVOKE_MODIFY: u8 = 1;
const INSTRUCTION_MODIFY_INVOKE: u8 = 2;
const INSTRUCTION_VERIFY_MODIFIED: u8 = 3;

solana_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    assert!(!accounts[ARGUMENT_INDEX].is_writable);

    match instruction_data[0] {
        INSTRUCTION_MODIFY => {
            msg!("modify ro account");
            assert_eq!(0, accounts[ARGUMENT_INDEX].try_borrow_data()?[0]);
            accounts[ARGUMENT_INDEX].try_borrow_mut_data()?[0] = 1;
        }
        INSTRUCTION_INVOKE_MODIFY => {
            msg!("invoke and modify ro account");

            assert_eq!(0, accounts[ARGUMENT_INDEX].try_borrow_data()?[0]);

            let instruction = Instruction {
                program_id: *program_id,
                accounts: vec![AccountMeta::new_readonly(
                    *accounts[ARGUMENT_INDEX].key,
                    false,
                )],
                data: vec![INSTRUCTION_MODIFY],
            };
            invoke(&instruction, accounts)?;
        }
        INSTRUCTION_MODIFY_INVOKE => {
            msg!("modify and invoke ro account");

            assert_eq!(0, accounts[ARGUMENT_INDEX].try_borrow_data()?[0]);
            accounts[ARGUMENT_INDEX].try_borrow_mut_data()?[0] = 1;

            let instruction = Instruction {
                program_id: *program_id,
                accounts: vec![AccountMeta::new_readonly(
                    *accounts[ARGUMENT_INDEX].key,
                    false,
                )],
                data: vec![INSTRUCTION_VERIFY_MODIFIED],
            };
            invoke(&instruction, accounts)?;
        }
        INSTRUCTION_VERIFY_MODIFIED => {
            msg!("verify modified");
            assert_eq!(1, accounts[ARGUMENT_INDEX].try_borrow_data()?[0])
        }
        _ => panic!("Unknown instruction"),
    }
    Ok(())
}
pub struct AccountInfo<'a> {
    pub key: &'a Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
    pub data: Rc<RefCell<&'a mut [u8]>>,
    pub owner: &'a Pubkey,
    pub lamports: &'a mut u64,
    pub executable: bool,
    pub rent_epoch: Epoch,
}
let account_info = AccountInfo {
    key: &key,
    is_signer: false,
    is_writable: true,
    data: Rc::new(RefCell::new(&mut data)),
    owner: &owner,
    lamports: &mut lamports,
    executable: false,
    rent_epoch: 0,
};
// Assume that the data length is stored in the RO segment of the BPF ELF.
const DATA_LEN: usize = 128;

// Allocate a buffer in RW memory to store the data.
let mut data = vec![0u8; DATA_LEN];

// Get a raw pointer to the data buffer.
let data_ptr = data.as_mut_ptr();

// Create a slice from the raw pointer and length.
let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, DATA_LEN) };

// Create a reference-counted pointer to the slice.
let data_rc = Rc::new(RefCell::new(data_slice));

// Use the reference-counted pointer to create an AccountInfo object.
let account_info = AccountInfo {
    key: &key,
    is_signer: false,
    is_writable: true,
    data: data_rc,
    owner: &owner,
    lamports: &mut lamports,
    executable: false,
    rent_epoch: 0,
};
