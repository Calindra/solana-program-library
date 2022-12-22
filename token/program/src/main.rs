pub(crate) use std::io;
use spl_token::{processor::Processor};
use cartesi_solana::{adapter::{get_processor_args, persist_accounts}, cartesi_stub::CartesiStubs};

fn main() -> io::Result<()> {
    let (program_id, accounts, data, last_instruction) = get_processor_args();
    solana_program::program_stubs::set_syscall_stubs(Box::new(CartesiStubs { program_id: program_id.clone() }));

    Processor::process(&program_id, &accounts, &data).unwrap();
    persist_accounts(&accounts, last_instruction);
    Ok(())
}
