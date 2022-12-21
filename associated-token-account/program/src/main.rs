use cartesi_solana::adapter::{get_processor_args, persist_accounts};
use serde::{Serialize, Deserialize};
use solana_program::{
    account_info::AccountInfo,
    instruction::Instruction,
    program_error::{PrintProgramError, ProgramError},
    program_stubs::SyscallStubs,
    pubkey::Pubkey, stake_history::Epoch,
};
pub(crate) use std::io;
use std::io::Write;
use std::process::{Child, Command, Stdio};

fn main() -> io::Result<()> {
    solana_program::program_stubs::set_syscall_stubs(Box::new(CartesiStubs {}));

    let (program_id, accounts, data, last_instruction) = get_processor_args();
    spl_associated_token_account::processor::process_instruction(&program_id, &accounts, &data);

    persist_accounts(&accounts, last_instruction);
    Ok(())
}

fn execute_spawn(program_id: String) -> Child {
    Command::new(format!("./solana_smart_contract_bin/{:?}", program_id))
        .stdin(Stdio::piped())
        .spawn()
        .unwrap()
}

#[derive(Serialize, Deserialize, Clone)]
struct AccountInfoSerialize {
  pub key: Pubkey,
  pub is_signer: bool,
  pub is_writable: bool,
  pub lamports: u64,
  pub data: Vec<u8>,
  pub owner: Pubkey,
  pub executable: bool,
  pub rent_epoch: Epoch,
}


struct CartesiStubs {}
impl SyscallStubs for CartesiStubs {
    fn sol_invoke_signed(
        &self,
        instruction: &Instruction,
        account_infos: &[AccountInfo], // chaves publicas
        signers_seeds: &[&[&[u8]]],
    ) -> Result<(), ProgramError> {
        // @todo validate signers_seeds
        // println!(
        //     "sol_invoke_signed {:?} {:?} {:?}",
        //     instruction, account_infos, signers_seeds
        // );
        let mut child = execute_spawn(instruction.program_id.to_string());
        let child_stdin = child.stdin.as_mut().unwrap();
        let instruction = bincode::serialize(&instruction).unwrap();

        let accounts_encoded: Vec<AccountInfoSerialize> = account_infos.into_iter().map(
          |account| AccountInfoSerialize {
            key: account.key.to_owned(),
            is_signer: account.is_signer,
            is_writable: account.is_writable,
            owner: account.owner.to_owned(),
            lamports: account.lamports.borrow_mut().to_owned(),
            data: account.data.borrow_mut().to_vec(),
            executable: account.executable,
            rent_epoch: account.rent_epoch,
          }
        ).collect();

        let accounts_binary = bincode::serialize(&accounts_encoded).unwrap();

        let signers_seeds = bincode::serialize(&signers_seeds).unwrap();

        child_stdin.write_all(b"Header: CPI")?;
        child_stdin.write_all(b"\n")?;

        child_stdin.write_all(&instruction)?;
        child_stdin.write_all(b"\n")?;
        child_stdin.write_all(&accounts_binary)?;
        child_stdin.write_all(b"\n")?;
        child_stdin.write_all(&signers_seeds)?;
        child_stdin.write_all(b"\n")?;

        drop(child_stdin);

        let output = child.wait_with_output()?;
        println!("output: {:?}", output);

        let exit_code = output.status.code();

        match exit_code {
            None => {
                println!("Program failed to run");
                return Err(ProgramError::Custom(1));
            }
            Some(code) => {
                if code == 0 {
                    println!("Program exited with success code");
                } else {
                    println!("Program exited with error code: {}", code);
                    return Err(ProgramError::Custom(1));
                }
            }
        }

        Ok(())
    }

    fn sol_get_return_data(&self) -> Option<(Pubkey, Vec<u8>)> {
        println!("sol_get_return_data");
        None
    }
}
