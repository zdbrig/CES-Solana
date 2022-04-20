use solana_program::account_info::next_account_info;
use solana_program::program_error::ProgramError;
use solana_program::{entrypoint::ProgramResult, msg};
use solana_program::pubkey::Pubkey;
//use  solana_program::msg;
use solana_program::entrypoint;
use solana_program::sysvar::slot_history::AccountInfo;
use borsh::{BorshDeserialize, BorshSerialize};
//use solana_program::account_info::next_account_info;
entrypoint!(ces_process);

#[derive(BorshDeserialize , BorshSerialize , Debug)]
pub struct AddressType {
   pub id: u8,
   pub mint: Pubkey,
   pub min_votes: u32,
   pub max_votes: u32,
   pub min_report: u32,
   pub collateral: u32,
   pub held_address: Pubkey,
   pub voters: [Pubkey; 16],
   pub reports: u16,
}

#[derive(BorshDeserialize , BorshSerialize , Debug)]
pub struct TransactionVote {
   pub id: u8,
   pub sender: Pubkey,
   pub amount: u64,
   pub state: u8,
   pub address_type: Pubkey,
   pub votes: u16,
}

pub fn ces_process(

   _program_id: &Pubkey,
   accounts: &[AccountInfo],
   instructions: &[u8]
) -> ProgramResult {

   let accounts_iter = &mut accounts.iter();
   let account = next_account_info(accounts_iter)?;

   let instruction_nubmer = instructions[0];

   match instruction_nubmer {
      1 => {
         let mut transactionVote = TransactionVote::try_from_slice(&instructions[1..])?;

         msg!("  id = {}" , transactionVote.id );
         msg!("  sender = {}" , transactionVote.sender );
         msg!("  amount = {}" , transactionVote.amount );
         msg!("  state = {}" , transactionVote.state );
         msg!("  address_type = {}" , transactionVote.address_type );
         msg!(" votes = {}" , transactionVote.votes);

           transactionVote.amount = 150;
            transactionVote.votes = 5;
         transactionVote.serialize(&mut &mut account.data.borrow_mut() [..])?;
      } ,
      _ => {
        return Err(ProgramError::InvalidInstructionData);
      }
   }


   Ok(())
}
