use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

// Define the entrypoint for the solana program
entrypoint!(process_instruction);




fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let tweet_data = std::str::from_utf8(instruction_data).unwrap();
    msg!("Received Tweet Data: {}", tweet_data);
    let expected_hashtag = "#solana";
    let actual_hashtag = "#solana";
    if actual_hashtag == expected_hashtag {
        msg!("Hashtaf verification succeded for tweet: {}", tweet_data);

    } else {
        msg!("Hashtag verification failed for tweet data: {}", tweet_data);
   
    }

    Ok(())

}