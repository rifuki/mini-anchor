use mini_anchor::{AnchorDeserialize, AnchorSerialize};
use solana_program::pubkey::Pubkey;

// Helper function
fn assert_roundtrip<T>(value: T, expected_size: usize)
where
    T: AnchorSerialize + AnchorDeserialize + PartialEq + std::fmt::Debug,
{
    let mut buf = vec![0u8; 1024];

    let written = value.serialize(&mut buf).unwrap();
    let (decoded, read) = T::deserialize(&buf[..written]).unwrap();

    assert_eq!(
        written, expected_size,
        "Serialized size does not match expected size"
    );
    assert_eq!(
        read, expected_size,
        "Deserialized size does not match expected size"
    );
    assert_eq!(decoded, value);
}

// ==============================
// Roundtrip
// ==============================
#[test]
fn test_u8() {
    assert_roundtrip(0u8, 1);
    assert_roundtrip(u8::MAX, 1);
}

#[test]
fn test_i8() {
    assert_roundtrip(i8::MIN, 1);
    assert_roundtrip(i8::MAX, 1);
}

#[test]
fn test_u16() {
    assert_roundtrip(0u16, 2);
    assert_roundtrip(u16::MAX, 2);
}

#[test]
fn test_i16() {
    assert_roundtrip(i16::MIN, 2);
    assert_roundtrip(i16::MAX, 2);
}

#[test]
fn test_u32() {
    assert_roundtrip(0u32, 4);
    assert_roundtrip(u32::MAX, 4);
}

#[test]
fn test_i32() {
    assert_roundtrip(i32::MIN, 4);
    assert_roundtrip(i32::MAX, 4);
}

#[test]
fn test_u64() {
    assert_roundtrip(0u64, 8);
    assert_roundtrip(u64::MAX, 8);
}

#[test]
fn test_i64() {
    assert_roundtrip(i64::MIN, 8);
    assert_roundtrip(i64::MAX, 8);
}

#[test]
fn test_u128() {
    assert_roundtrip(0u128, 16);
    assert_roundtrip(u128::MAX, 16);
}

#[test]
fn test_i128() {
    assert_roundtrip(i128::MIN, 16);
    assert_roundtrip(i128::MAX, 16);
}

#[test]
fn test_bool() {
    assert_roundtrip(true, 1);
    assert_roundtrip(false, 1);
}

#[test]
fn test_string() {
    assert_roundtrip("".to_string(), 4);
    assert_roundtrip("hello".to_string(), 4 + 5);
    assert_roundtrip("39".repeat(10), 4 + 2 * 10);
}

#[test]
fn test_option_some() {
    assert_roundtrip(Some(123u64), 1 + 8);
    assert_roundtrip(Some(true), 1 + 1);
    assert_roundtrip(Some("miku".to_string()), 1 + 4 + 4);
}

#[test]
fn test_option_none() {
    assert_roundtrip(None::<u64>, 1);
    assert_roundtrip(None::<bool>, 1);
    assert_roundtrip(None::<String>, 1);
}

#[test]
fn test_vec() {
    assert_roundtrip(Vec::<u32>::new(), 4);
    assert_roundtrip(vec![1u8, 2, 3], 4 + 3);
    assert_roundtrip(vec![100u64; 20], 4 + (8 * 20));
}

#[test]
fn test_pubkey() {
    assert_roundtrip(Pubkey::new_from_array([0u8; 32]), 32);
    assert_roundtrip(Pubkey::new_from_array([u8::MAX; 32]), 32);
}

// ==============================
// Combined tests
// =============================
#[test]
fn test_combined_primitives() {
    let a: u8 = 0xFF;
    let b: i16 = -1000;
    let c: u32 = 12345;
    let d: i64 = -99999;
    let e: u128 = 1234567890123;
    let f: bool = true;
    let expected_size = 1 + 2 + 4 + 8 + 16 + 1;

    let mut buf = [0u8; 32];
    let mut write_offset = 0;

    write_offset += a.serialize(&mut buf[write_offset..]).unwrap();
    write_offset += b.serialize(&mut buf[write_offset..]).unwrap();
    write_offset += c.serialize(&mut buf[write_offset..]).unwrap();
    write_offset += d.serialize(&mut buf[write_offset..]).unwrap();
    write_offset += e.serialize(&mut buf[write_offset..]).unwrap();
    write_offset += f.serialize(&mut buf[write_offset..]).unwrap();

    // Expected size calculation = 1 + 2 + 4 + 8 + 16 + 1 = 32
    assert_eq!(write_offset, expected_size);

    // Deserialization
    let mut read_offset = 0;
    let (a_dec, read_bytes) = u8::deserialize(&buf[read_offset..]).unwrap();
    read_offset += read_bytes;
    let (b_dec, read_bytes) = i16::deserialize(&buf[read_offset..]).unwrap();
    read_offset += read_bytes;
    let (c_dec, read_bytes) = u32::deserialize(&buf[read_offset..]).unwrap();
    read_offset += read_bytes;
    let (d_dec, read_bytes) = i64::deserialize(&buf[read_offset..]).unwrap();
    read_offset += read_bytes;
    let (e_dec, read_bytes) = u128::deserialize(&buf[read_offset..]).unwrap();
    read_offset += read_bytes;
    let (f_dec, read_bytes) = bool::deserialize(&buf[read_offset..]).unwrap();
    read_offset += read_bytes;

    assert_eq!(read_offset, write_offset);
    assert_eq!(a, a_dec);
    assert_eq!(b, b_dec);
    assert_eq!(c, c_dec);
    assert_eq!(d, d_dec);
    assert_eq!(e, e_dec);
    assert_eq!(f, f_dec);
}

#[test]
fn test_nested_option_vec() {
    let value: Option<Vec<Option<u16>>> = Some(vec![Some(100), None, Some(200)]);
    let expected_size = 1 + (4 + ((1 + 2) + 1 + (1 + 2)));

    let mut buf = vec![0u8; 1024];

    let write_offset = value.serialize(&mut buf).unwrap();
    assert_eq!(write_offset, expected_size);

    let (value_dec, read_offset) =
        Option::<Vec<Option<u16>>>::deserialize(&buf[..write_offset]).unwrap();
    assert_eq!(read_offset, write_offset);
    assert_eq!(value, value_dec);
}

#[test]
fn test_combined_mixed_types() {
    let owner: Pubkey = Pubkey::new_from_array([1u8; 32]);
    let name: String = "Hatsune".to_string();
    let balance: u64 = 39;
    let is_vip: bool = true;
    let refferer: Option<Pubkey> = Some(Pubkey::new_from_array([3u8; 32]));
    let tags: Vec<String> = vec!["music".to_string(), "vocaloid".to_string()];

    let expected_size = 32 + (4 + 7) + 8 + 1 + (1 + 32) + (4 + (4 + 5) + (4 + 8));

    let mut buf = [0u8; 256];
    let mut write_offset = 0;

    write_offset += owner.serialize(&mut buf[write_offset..]).unwrap();
    write_offset += name.serialize(&mut buf[write_offset..]).unwrap();
    write_offset += balance.serialize(&mut buf[write_offset..]).unwrap();
    write_offset += is_vip.serialize(&mut buf[write_offset..]).unwrap();
    write_offset += refferer.serialize(&mut buf[write_offset..]).unwrap();
    write_offset += tags.serialize(&mut buf[write_offset..]).unwrap();

    // Expected size calculation = 32 + (4 + 7) + 8 + 1 + (1 + 32) + (4 + (4 + 5) + (4 + 8)) = 104
    assert_eq!(write_offset, expected_size);

    // Deserialization
    let mut read_offset = 0;
    let (owner_dec, read_bytes) = Pubkey::deserialize(&buf[read_offset..]).unwrap();
    read_offset += read_bytes;
    let (name_dec, read_bytes) = String::deserialize(&buf[read_offset..]).unwrap();
    read_offset += read_bytes;
    let (balance_dec, read_bytes) = u64::deserialize(&buf[read_offset..]).unwrap();
    read_offset += read_bytes;
    let (is_vip_dec, read_bytes) = bool::deserialize(&buf[read_offset..]).unwrap();
    read_offset += read_bytes;
    let (refferer_dec, read_bytes) = Option::<Pubkey>::deserialize(&buf[read_offset..]).unwrap();
    read_offset += read_bytes;
    let (tags_dec, read_bytes) = Vec::<String>::deserialize(&buf[read_offset..]).unwrap();
    read_offset += read_bytes;

    assert_eq!(read_offset, write_offset);
    assert_eq!(owner, owner_dec);
    assert_eq!(name, name_dec);
    assert_eq!(balance, balance_dec);
    assert_eq!(is_vip, is_vip_dec);
    assert_eq!(refferer, refferer_dec);
    assert_eq!(tags, tags_dec);
}

// ==============================
// Error cases
// ============================
#[test]
fn test_error_buffer_too_small() {
    let mut buf = [0u8; 1];
    assert!(u16::MAX.serialize(&mut buf).is_err());
}

#[test]
fn test_error_data_too_short() {
    let buf = [1u8; 2];
    assert!(u32::deserialize(&buf).is_err());
}
