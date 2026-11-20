//! Shared code: a zstd round trip, which needs the C toolchain on every OS.

/// Compress and decompress a fixed payload, and describe the result.
pub fn compressed_roundtrip() -> String {
    let payload = b"hyperi-ci windows and macos legs".repeat(64);
    let packed = zstd::encode_all(&payload[..], 3).expect("zstd encodes");
    let unpacked = zstd::decode_all(&packed[..]).expect("zstd decodes");
    assert_eq!(unpacked, payload);
    format!("zstd {} -> {} bytes", payload.len(), packed.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_shrinks_a_repeated_payload() {
        assert!(compressed_roundtrip().starts_with("zstd 2048 -> "));
    }
}
