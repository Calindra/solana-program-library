use cartesi_solana::executor::create_executor;
use spl_token::processor::Processor;
pub(crate) use std::io;

fn main() -> io::Result<()> {
    let mut executor = create_executor();
    executor.get_processor_args(|program_id, accounts, data| {
        Processor::process(&program_id, &accounts, &data).unwrap();
    });
    Ok(())
}
