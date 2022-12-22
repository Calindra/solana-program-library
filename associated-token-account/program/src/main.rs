use cartesi_solana::{
    adapter::{get_processor_args, persist_accounts},
    cartesi_stub::CartesiStubs,
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
