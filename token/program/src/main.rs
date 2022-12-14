pub(crate) use std::io;
use spl_token::{error::TokenError, processor::Processor};
use cartesi_solana::adapter::{get_processor_args, persist_accounts};
use solana_program::{pubkey::Pubkey, account_info::AccountInfo, program_error::PrintProgramError};
use cartesi_solana::anchor_lang::solana_program::entrypoint::ProgramResult;

fn main() -> io::Result<()> {
    let (program_id, accounts, data, last_instruction) = get_processor_args();
    Processor::process(&program_id, &accounts, &data).unwrap();
    persist_accounts(&accounts, last_instruction);
    Ok(())
}
