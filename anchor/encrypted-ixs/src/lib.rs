use arcis_imports::*;

#[encrypted]
mod circuits {
    use arcis_imports::*;

    /// Confidential DeFi account state.
    /// Owner pubkey is split into two u128s because Arcis encrypts each primitive separately.
    pub struct PrivateAccount {
        pub owner_lo: u128,  // Pubkey lower 128 bits
        pub owner_hi: u128,  // Pubkey upper 128 bits
        pub balance: u64,
        pub token_mint: u64, // Mock/hash of token mint
    }

    /// Initialize a new private account with zero state.
    /// Returns encrypted state owned by the MXE.
    #[instruction]
    pub fn init_account(mxe: Mxe) -> Enc<Mxe, PrivateAccount> {
        let initial_state = PrivateAccount {
            owner_lo: 0,
            owner_hi: 0,
            balance: 0,
            token_mint: 0,
        };
        mxe.from_arcis(initial_state)
    }
}
