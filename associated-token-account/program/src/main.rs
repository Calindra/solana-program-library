pub(crate) use std::io;
use cartesi_solana::adapter::{get_processor_args, persist_accounts};
use solana_program::{pubkey::Pubkey, account_info::AccountInfo, program_error::PrintProgramError};

fn main() -> io::Result<()> {
    let (program_id, accounts, data, last_instruction) = get_processor_args();
    spl_associated_token_account::processor::process_instruction(&program_id, &accounts, &data).unwrap();
    persist_accounts(&accounts, last_instruction);
    Ok(())
}
