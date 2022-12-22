use cartesi_solana::{
    adapter::{get_processor_args, persist_accounts},
    cartesi_stub::CartesiStubs,
};

pub(crate) use std::io;

fn main() -> io::Result<()> {
    let (program_id, accounts, data, last_instruction) = get_processor_args();

    solana_program::program_stubs::set_syscall_stubs(Box::new(CartesiStubs { program_id: program_id.clone() }));

    spl_associated_token_account::processor::process_instruction(&program_id, &accounts, &data)
        .unwrap();

    persist_accounts(&accounts, last_instruction);
    Ok(())
}
