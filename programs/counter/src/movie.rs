use anchor_lang::prelude::*;

const MIN_RATING: u8 = 1;
const MAX_RATING: u8 = 5;
const MAX_TITLE_LENGTH: usize = 20;
const MAX_DESCRIPTION_LENGTH: usize = 50;
const DISCRIMINATOR: usize = 8;

declare_id!("AC7s1aCWH8gtYXLhLSJbWzz3xytNmC5EaLZKG4D73ohx");

#[program]
pub mod movie {
    use super::*;

    pub fn add_movie_review(ctx: Context<AddMovieReview>, title: String, description: String, rating: u8) -> Result <()> {
        require!(rating >= MIN_RATING && rating <= MAX_RATING, MovieReviewError::InvalidRating);
        require!(title.len() <= MAX_TITLE_LENGTH, MovieReviewError::TitleTooLong);
        require!(description.len() <= MAX_DESCRIPTION_LENGTH, MovieReviewError::DescriptionTooLong);
 
        msg!("Movie Review Account Created");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.rating = rating;
        movie_review.description = description;
        Ok(())
    }

    pub fn update_movie_review(ctx: Context<UpdateMovieReview>, title: String, description: String, rating: u8) -> Result <()> {
        msg!("Movie review account space reallocated");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.rating = rating;
        movie_review.description = description;
        
        Ok(())
    }

    pub fn delete_movie_review(_ctx: Context<DeleteMovieReview>, title: String) -> Result<()> {
        msg!("Movie review for {} deleted", title);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String, description:String)]
pub struct AddMovieReview<'info> {
    #[account(
        init, // defini que irá criar
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump, 
        payer = initializer, 
        space = DISCRIMINATOR + MovieAccountState::INIT_SPACE
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer:  Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title: String, description:String)]
pub struct UpdateMovieReview<'info> {
    #[account(
        mut, // defini que irá atualizar
        seeds = [title.as_bytes(), initializer.key().as_ref()], 
        bump, 
        realloc = DISCRIMINATOR + MovieAccountState::INIT_SPACE, 
        realloc::payer = initializer, // onta para subtrair ou adicionar + para quem
        realloc::zero = true // especificar se a nova memória deve ser inicializada em zero
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer:  Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, description:String)]
pub struct DeleteMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()], 
        bump,
        close=initializer // fecha a conta e o aluguel é reembolsado para o initializer
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer:  Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)] // usado para calcular o INIT_SPACE -> representa o espaço necessário para o campos de conta
pub struct MovieAccountState {
    pub reviewer: Pubkey, // 32
    pub rating: u8, // 1.
    #[max_len(20)]
    pub title: String, // 4 + len()
    #[max_len(50)]
    pub description: String // 4 + len()
}

#[error_code] // gerar tipos de erro para serem usados como tipos de retorno
enum MovieReviewError {
    #[msg("Rating must be between 1 and 5")]
    InvalidRating,
    #[msg("Movie Title too long")]
    TitleTooLong,
    #[msg("Movie Description too long")]
    DescriptionTooLong,
}
