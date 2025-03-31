#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("DAYEaUQ6mxZVyY6Mz4xpoh7kGTGGGDSXWccSfhfdq7kc");

#[program]
pub mod my_notes_app {
    use super::*;

    // Implementing the logic or program actions
    pub fn add_note(ctx: Context<AddingNote>, title: String, text: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        let author = &ctx.accounts.author; // The author account
        let clock = Clock::get().unwrap(); // Getting the current timestamp

        if text.chars().count() > 400 || title.chars().count() > 64 {
            return Err(ErrorCode::TextTooLong.into());
        }

        note.author = *author.key;
        note.title = title;
        note.created_at = clock.unix_timestamp;
        note.updated_at = clock.unix_timestamp;
        note.text = text;

        Ok(())
    }

    pub fn update_note(ctx: Context<UpdatingNote>, text: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();

        if text.chars().count() > 400 {
            return Err(ErrorCode::TextTooLong.into());
        }

        note.author = *author.key;
        note.text = text;
        note.updated_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn delete_note(ctx: Context<DeletingNote>) -> Result<()> {
        let note = &mut ctx.accounts.note;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();

        note.author = *author.key;
        note.title = "".to_string();
        note.text = "".to_string();
        note.updated_at = clock.unix_timestamp;
        Ok(())
    }
}

/// Listing all the accounts necessary for the instructions to do their job.
/// This is the initialize struct created when we initialized our project
/// we have renamed it here to `AddingNote<'info>`
#[derive(Accounts)]
pub struct AddingNote<'info> {
    // Contraints below shows we are initializing, author will pay for the transaction 
    // and space will be LEN as we defined.
    #[account(init, payer = author, space = Note::LEN)] // providing constraints
    pub note: Account<'info, Note>,
    // Payer must be mutable as we will be changing the balance
    // of the account of the author when he pays.
    #[account(mut)] // Providing mutable constraint.
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatingNote<'info> {
    #[account(mut, has_one = author)]
    pub note: Account<'info, Note>,
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeletingNote<'info> {
    #[account(mut, has_one = author)]
    pub note: Account<'info, Note>,
    pub author: Signer<'info>,
}

/// Note data that we will be storing on-chain.
/// Note is an account since solana things are stored in accounts.
#[account]
pub struct Note {
    pub author: Pubkey, // Account that owns the note
    pub title: String, // This is the title or topic of the note.
    pub text: String, // Text describing the note.
    pub created_at: i64, // Timestamp of the note creation.
    pub updated_at: i64, // Timestamp of the note update.
}

/// Setting the space.
const DISCRIMINATOR: usize = 8; // Indicates the type or purpose of the account's data.
const PUBLIC_KEY_LENGTH: usize = 32;
const TITLE_LENGTH: usize = 4 + 65 * 4; // 64 chars.
const TEXT_LENGTH: usize = 4 + 400 * 4; // 400 chars.
const TIMESTAMP_LENGTH: usize = 8;

/// Defining LEN so we can access length of the note via calling Note::LEN
impl Note {
    const LEN: usize = DISCRIMINATOR + // discriminator
        PUBLIC_KEY_LENGTH + // author
        TITLE_LENGTH+ // title
        TEXT_LENGTH + // text
        TIMESTAMP_LENGTH + // created_at
        TIMESTAMP_LENGTH; // updated_at
}

/// Custom error code
#[error_code]
pub enum ErrorCode {
    #[msg("The text is too long")]
    TextTooLong,
}
