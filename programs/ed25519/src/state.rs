use anchor_lang::prelude::*;
#[account]
pub struct Counter {
    pub signer_count: u8
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SignatureTuple {
  pub public_key: Pubkey,
  pub signature: [u8;64],
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InstructionHeader {
  pub length: u8,
  pub padding: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SignatureOffset {
  pub signature_offset: u16,
  pub signature_instruction_index: u16,
  pub public_key_offset: u16,
  pub public_key_instruction_index: u16,
  pub message_data_offset: u16,
  pub message_data_size: u16,
  pub message_instruction_index: u16,
}
