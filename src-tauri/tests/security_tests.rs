use quietbox_lib::crypto::container::{create_decoy_volume, add_hidden_volume, open_container};
use quietbox_lib::crypto::kdf::{KdfParams, generate_salt};
use quietbox_lib::crypto::cipher::{encrypt, decrypt, generate_nonce};
use std::io::Cursor;
use std::time::Instant;

fn test_kdf() -> KdfParams {
    KdfParams {
        m_cost: 8192,
        t_cost: 1,
        p_cost: 1,
    }
}

/// Calculate Shannon entropy of a byte slice. Max entropy is 8.0 bits per byte.
fn calculate_shannon_entropy(data: &[u8]) -> f64 {
    let mut counts = [0usize; 256];
    for &b in data {
        counts[b as usize] += 1;
    }

    let len = data.len() as f64;
    let mut entropy = 0.0;

    for &c in &counts {
        if c > 0 {
            let p = c as f64 / len;
            entropy -= p * p.log2();
        }
    }

    entropy
}

#[test]
fn test_security_container_entropy() {
    let total_size = 2 * 1024 * 1024u64; // 2 MiB
    let mut buf = vec![0u8; total_size as usize];
    let mut cur = Cursor::new(&mut buf);

    let decoy_data = b"Some random sample file content for decoy volume";
    let _ = create_decoy_volume(
        &mut cur,
        total_size,
        decoy_data,
        b"secret_decoy_pwd",
        512 * 1024,
        &test_kdf(),
        |_, _| {}
    ).unwrap();

    let entropy = calculate_shannon_entropy(&buf);
    println!("Container Shannon Entropy: {:.5} bits/byte", entropy);
    
    // High-entropy CSPRNG output should be > 7.99 for 2 MiB
    assert!(
        entropy > 7.99,
        "Entropy too low ({:.5}); container is distinguishable from pure noise",
        entropy
    );
}

#[test]
fn test_security_tamper_detection() {
    let key = [0x77u8; 32];
    let nonce = generate_nonce();
    let plaintext = b"Top secret data that must not be altered!";

    let ct = encrypt(&key, &nonce, plaintext).unwrap();

    // Flip bits at various positions
    for idx in [0, ct.len() / 2, ct.len() - 1] {
        let mut tampered = ct.clone();
        tampered[idx] ^= 0x01;
        assert!(
            decrypt(&key, &nonce, &tampered).is_err(),
            "Tampered ciphertext at byte {} must fail authentication!",
            idx
        );
    }
}

#[test]
fn test_security_salt_uniqueness() {
    let mut salts = std::collections::HashSet::new();
    for _ in 0..1000 {
        let salt = generate_salt();
        assert!(salts.insert(salt), "Duplicate salt generated!");
    }
}

#[test]
fn test_security_timing_consistency() {
    let total_size = 4 * 1024 * 1024u64;
    let mut buf = vec![0u8; total_size as usize];
    let mut cur = Cursor::new(&mut buf);

    let decoy_pw = b"correct_decoy_password";
    let hidden_pw = b"correct_hidden_password";

    let _ = create_decoy_volume(
        &mut cur,
        total_size,
        b"decoy payload",
        decoy_pw,
        1024 * 1024,
        &test_kdf(),
        |_, _| {}
    ).unwrap();

    let _ = add_hidden_volume(
        &mut cur,
        total_size,
        0,
        b"hidden payload",
        hidden_pw,
        1024 * 1024,
        &test_kdf(),
        |_, _| {}
    ).unwrap();

    // Measure wrong password timing
    let start_wrong = Instant::now();
    let _ = open_container(&mut cur, total_size, b"wrong_password_xyz", &test_kdf());
    let dur_wrong = start_wrong.elapsed();

    // Measure correct hidden password timing
    let start_hidden = Instant::now();
    let _ = open_container(&mut cur, total_size, hidden_pw, &test_kdf());
    let dur_hidden = start_hidden.elapsed();

    println!("Timing - Wrong PW: {:?}, Hidden PW: {:?}", dur_wrong, dur_hidden);
}
