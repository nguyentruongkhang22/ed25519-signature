use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions::load_instruction_at_checked;
pub mod constant;
pub mod context;
pub mod error;
pub mod state;

use crate::constant::*;
use crate::context::*;
use crate::error::ErrorCode;
use crate::state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod ed25519 {
    use super::*;

    pub fn create_counter(
        _ctx: Context<CreateCounterContext>,
        _derivation_path: Vec<u8>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn compare_message_signature(
        _ctx: Context<CompareMessageSignatureContext>,
        message: Vec<u8>,
        signatures: Vec<SignatureTuple>,
        instruction_data: Vec<u8>,
    ) -> Result<()> {
        let instruction_data_to_compare =
            create_message_verification_instruction(&message, &signatures);
        require!(
            instruction_data == instruction_data_to_compare,
            ErrorCode::InvalidInput
        );

        Ok(())
    }

    pub fn validate_message_signature(
        ctx: Context<ValidateMessageSignatureContext>,
        message: Vec<u8>,
        signatures: Vec<SignatureTuple>,
    ) -> Result<()> {
        let sysvar_instructions = &ctx.accounts.instructions;

        let is_valid = verify_message_signatures(&message, &signatures, sysvar_instructions, 0);

        require!(is_valid, ErrorCode::InvalidInput);

        let counter = &mut ctx.accounts.counter;
        counter.signer_count = signatures.len() as u8;
        msg!("Total signer: {:?}", counter.signer_count);

        Ok(())
    }
}

pub fn create_message_verification_instruction(
    message: &Vec<u8>,
    signatures: &Vec<SignatureTuple>,
) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    let header = InstructionHeader {
        length: signatures.len() as u8,
        padding: 0,
    };
    let header_bytes = header.try_to_vec().unwrap();
    data.extend_from_slice(&header_bytes);

    let message_offset =
        HEADER_SIZE + (SIGNATURE_OFFSET_SIZE + SIGNATURE_TUPLE_SIZE) * signatures.len();

    for i in 0..signatures.len() {
        let sign_data_offset =
            HEADER_SIZE + SIGNATURE_OFFSET_SIZE * signatures.len() + SIGNATURE_TUPLE_SIZE * i;

        let sign_offset = SignatureOffset {
            signature_offset: (sign_data_offset + PUBKEY_SIZE) as u16,
            signature_instruction_index: u16::MAX,
            public_key_offset: sign_data_offset as u16,
            public_key_instruction_index: u16::MAX,
            message_data_offset: message_offset as u16,
            message_data_size: message.len() as u16,
            message_instruction_index: u16::MAX,
        };

        let sign_offset_bytes = sign_offset.try_to_vec().unwrap();
        data.extend_from_slice(&sign_offset_bytes);
    }

    for i in 0..signatures.len() {
        let signature = signatures.get(i).unwrap();
        let signature_bytes = signature.try_to_vec().unwrap();

        data.extend_from_slice(&signature_bytes);
    }

    data.extend_from_slice(message);
    data
}

pub fn verify_message_signatures(
    message: &Vec<u8>,
    signatures: &Vec<SignatureTuple>,
    sysvar_instructions: &AccountInfo,
    instruction_index: usize,
) -> bool {
    let data_to_compare = create_message_verification_instruction(message, signatures);
    let veri_sign_ix = load_instruction_at_checked(instruction_index, sysvar_instructions).unwrap();

    if veri_sign_ix.program_id != id() {
        msg!("veri_sign_ix.program_id = {:?}", veri_sign_ix.program_id);
        return false;
    }

    if veri_sign_ix.data.to_owned() != data_to_compare {
        msg!("veri_sign_ix.data = {:?}", &veri_sign_ix.data);
        return false;
    }

    true
}
