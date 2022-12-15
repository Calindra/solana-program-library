use cartesi_solana::adapter::{get_processor_args, persist_accounts};
use solana_program::{
    account_info::AccountInfo, program_error::PrintProgramError, program_stubs::SyscallStubs,
    pubkey::Pubkey,
};
pub(crate) use std::io;

fn main() -> io::Result<()> {
    solana_program::program_stubs::set_syscall_stubs(Box::new(CartesiStubs {}));

    let (program_id, accounts, data, last_instruction) = get_processor_args();
    spl_associated_token_account::processor::process_instruction(&program_id, &accounts, &data)
        .unwrap();
    persist_accounts(&accounts, last_instruction);
    Ok(())
}

struct CartesiStubs {}
impl SyscallStubs for CartesiStubs {
    fn sol_invoke_signed(
        &self,
        instruction: &solana_program::instruction::Instruction,
        account_infos: &[AccountInfo],
        signers_seeds: &[&[&[u8]]],
    ) -> Result<(), solana_program::program_error::ProgramError> {
        println!("sol_invoke_signed {:?} {:?} {:?}", instruction, account_infos, signers_seeds);
        Ok(())
    }

    fn sol_get_return_data(&self) -> Option<(Pubkey, Vec<u8>)> {
        println!("sol_get_return_data");
        None
    }
}
