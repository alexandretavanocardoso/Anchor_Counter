use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, MintTo, Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;

// mint_to leva um CpiContext e inteiro como argumentos, onde o inteiro representa o número de tokens a cunhar
// MintTo pode ser usado para a lista de contas que a instrução de mint precisa.

mod constants;

use constants::*;

declare_id!("8zLkkwXZ6wjdBHKqwzYHjjZu8VLnpcRSyxfZKV6wJhUA");

#[program]
pub mod anchor_movie_review_program {
    use super::*;

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        // We require that the rating is between 1 and 5
        require!(rating >= MIN_RATING && rating <= MAX_RATING, MovieReviewError::InvalidRating);

        // We require that the title is not longer than 20 characters
        require!(title.len() <= MAX_TITLE_LENGTH, MovieReviewError::TitleTooLong);

        // We require that the description is not longer than 50 characters
        require!(description.len() <= MAX_DESCRIPTION_LENGTH, MovieReviewError::DescriptionTooLong);

        msg!("Movie review account created");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.description = description;
        movie_review.rating = rating;

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info()
                },
                &[&[
                    "mint".as_bytes(),
                    &[ctx.bumps.mint]
                ]]
            ),
            10*10^6
        )?;

        msg!("Minted tokens");

        Ok(())
    }

    pub fn update_movie_review(
        ctx: Context<UpdateMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        // We require that the rating is between 1 and 5
        require!(rating >= MIN_RATING && rating <= MAX_RATING, MovieReviewError::InvalidRating);

        // We require that the title is not longer than 20 characters
        require!(title.len() <= MAX_TITLE_LENGTH, MovieReviewError::TitleTooLong);

        // We require that the description is not longer than 50 characters
        require!(description.len() <= MAX_DESCRIPTION_LENGTH, MovieReviewError::DescriptionTooLong);

        msg!("Movie review account space reallocated");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.description = description;
        movie_review.rating = rating;

        Ok(())
    }

    pub fn delete_movie_review(_ctx: Context<DeleteMovieReview>, title: String) -> Result<()> {
        msg!("Movie review for {} deleted", title);
        Ok(())
    }

    pub fn initialize_token_mint(_ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Token mint initialized");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct AddMovieReview<'info> {
    #[account(
        init,
        seeds=[title.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = MovieAccountState::INIT_SPACE + title.len() + description.len(), // We add the length of the title and description to the init space
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>, // usaremos o Programa Token para cunhar tokens
    #[account(
        seeds = ["mint".as_bytes()],
        bump,
        mut
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed, // se a conta ainda não tiver foi inicializado, será inicializado
        payer = initializer,
        associated_token::mint = mint,  // garante que o a conta é inicializada o mint
        associated_token::authority = initializer  // garante que o a conta é inicializada com as autoridades especificadas
    )]
    pub token_account: Account<'info, TokenAccount>, // a conta de token associada para o mint
    pub associated_token_program: Program<'info, AssociatedToken>, //  necessário porque usaremos o associated_token
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateMovieReview<'info> {
    #[account(
        mut,
        seeds=[title.as_bytes(), initializer.key().as_ref()],
        bump,
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteMovieReview<'info> {
    #[account(
        mut,
        seeds=[title.as_bytes(), initializer.key().as_ref()],
        bump,
        close=initializer
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = user,
        mint::decimals = 6, // garante que o a conta é inicializada como uma nova moeda de token com os decimais
        mint::authority = user, // garante que o a conta é inicializada como uma nova moeda de token com o conjunto de autoridade
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MovieAccountState {
    pub reviewer: Pubkey,
    pub rating: u8,
    pub title: String,
    pub description: String,
}

impl Space for MovieAccountState {
    const INIT_SPACE: usize = ANCHOR_DISCRIMINATOR + PUBKEY_SIZE + U8_SIZE + STRING_LENGTH_PREFIX + STRING_LENGTH_PREFIX;
}

#[error_code]
enum MovieReviewError {
    #[msg("Rating must be between 1 and 5")]
    InvalidRating,
    #[msg("Movie Title too long")]
    TitleTooLong,
    #[msg("Movie Description too long")]
    DescriptionTooLong,
}