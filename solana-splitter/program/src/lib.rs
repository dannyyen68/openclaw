//! SOL-Splitter: Verteilt das gesamte SOL-Guthaben eines Vault-PDAs
//! zu je 1/3 an drei feste Empfänger.
//!
//! Ablauf:
//!  1. Jeder überweist SOL an die Vault-Adresse (PDA, seeds = ["vault"]).
//!  2. Irgendjemand ruft das Programm auf (Instruction-Daten leer) -> Verteilung.
//!
//! Accounts: [0] vault PDA (writable), [1..=3] Empfänger (writable)
//! Die Empfänger-Adressen sind unten fest einkompiliert.
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

// Empfänger (öffentliche Adressen aus generate-wallets.mjs / .env)
const RECIPIENTS: [Pubkey; 3] = [
    pubkey!("n9h1A1wUteC9W7ziwQdLJpCVZgKNi6eabB2t2iZwrzm"),
    pubkey!("5TsaujNUhZyxC5t8Fz8vwAiQMvWgSjD7vncM6k5d5Zug"),
    pubkey!("B8Mn3bDHWfSqqjM8BDcU4b4gHbwQeNKGUwwSM6ZdUeeS"),
];

entrypoint!(process);

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let it = &mut accounts.iter();
    let vault = next_account_info(it)?;
    let (expected, _bump) = Pubkey::find_program_address(&[b"vault"], program_id);
    if *vault.key != expected {
        return Err(ProgramError::InvalidSeeds);
    }

    let recipients = [next_account_info(it)?, next_account_info(it)?, next_account_info(it)?];
    for (acc, want) in recipients.iter().zip(RECIPIENTS.iter()) {
        if acc.key != want {
            return Err(ProgramError::InvalidArgument);
        }
    }

    // Das Vault-PDA gehört dem System Program, daher kann das Programm nicht
    // direkt Lamports abbuchen. Darum: Vault muss dem Programm gehören.
    // -> Beim ersten Aufruf per `CreateAccount`/`assign` an das Programm übertragen
    //    (siehe client/init.mjs). Dann dürfen wir Lamports direkt verschieben.
    if vault.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    let rent_min = Rent::get()?.minimum_balance(vault.data_len());
    let distributable = vault.lamports().saturating_sub(rent_min);
    let share = distributable / 3;
    if share == 0 {
        return Err(ProgramError::InsufficientFunds);
    }

    for r in recipients.iter() {
        **vault.try_borrow_mut_lamports()? -= share;
        **r.try_borrow_mut_lamports()? += share;
    }
    Ok(())
}
