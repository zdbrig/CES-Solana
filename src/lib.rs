use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use  solana_program::msg;
use solana_program::entrypoint;
use solana_program::sysvar::slot_history::AccountInfo;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::next_account_info;
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
   _accounts: &[AccountInfo],
   _instructions: &[u8]
) -> ProgramResult {

   Ok(())
}
