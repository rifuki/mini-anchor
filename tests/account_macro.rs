use mini_anchor::account;
use solana_program::pubkey::Pubkey;

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub balance: u64,
    pub is_active: bool,
    #[max_len(32)]
    pub name: String,
}

#[test]
fn test_space() {
    assert_eq!(UserProfile::SPACE, 8 + 32 + 8 + 1 + (4 + 32))
}

#[test]
fn test_discriminator() {
    assert_eq!(UserProfile::DISCRIMINATOR.len(), 8);
}

#[test]
fn test_roundtrip() {
    let user_profile = UserProfile {
        authority: Pubkey::new_unique(),
        balance: 1000,
        is_active: true,
        name: "Alice".to_string(),
    };

    let mut buf = vec![0u8; UserProfile::SPACE];
    user_profile.try_serialize(&mut buf).unwrap();

    let decoded = UserProfile::try_deserialize(&buf).unwrap();

    assert_eq!(decoded.authority, user_profile.authority);
    assert_eq!(decoded.balance, user_profile.balance);
    assert_eq!(decoded.is_active, user_profile.is_active);
    assert_eq!(decoded.name, user_profile.name);
}
