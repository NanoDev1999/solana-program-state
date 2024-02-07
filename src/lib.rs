use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    account_info::AccountInfo,
    pubkey::Pubkey,
    msg
};

pub mod instruction;
use instruction::MovieInstruction;


entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult{
    msg!("process_instruction() called");

    // Unpack called
    let instruction = MovieInstruction::unpack(instruction_data)?;
    // Match against the data struct returned into `instruction` variable
    match instruction {
        MovieInstruction::AddMovieReview { title, rating, description } => {
            // Make a call to `add_move_review` function
            let add_movie_review_result = add_movie_review(program_id, accounts, title, rating, description);
            msg!("add_movie_review_result: {:?}", add_movie_review_result);
        }
    }




    Ok(())
}



pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String
) -> ProgramResult {

    msg!("add_movie_review() called");

    // Logging instruction data that was passed in
    msg!("Adding movie review...");
    msg!("Title: {}", title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", description);

    Ok(())
}










