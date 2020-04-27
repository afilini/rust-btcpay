use std::ops::Deref;

use bitcoin::util::base58;

use bitcoin::secp256k1::{
    Error as Secp256k1Error, Message, PublicKey, Secp256k1, SecretKey, SerializedSignature,
};

use bitcoin::hashes::ripemd160::Hash as Ripemd160Hash;
use bitcoin::hashes::sha256::Hash as Sha256Hash;
use bitcoin::hashes::Hash;

#[derive(Debug, Clone)]
pub struct Cryptography;

impl Cryptography {
    pub fn generate_keypair() -> KeyPair {
        Secp256k1::signing_only()
            .generate_keypair(&mut bitcoin::secp256k1::rand::thread_rng())
            .into()
    }

    pub fn load_keypair(secret_key: SecretKey) -> KeyPair {
        let public_key = PublicKey::from_secret_key(&Secp256k1::signing_only(), &secret_key);

        KeyPair {
            secret: secret_key,
            public: public_key,
        }
    }

    pub fn get_sin_from_key(public_key: &PublicKey) -> String {
        let version = Self::get_version_from_key(public_key);
        let checksum = Self::get_checksum_from_version(&version);

        let mut buffer = version.to_vec();
        buffer.extend_from_slice(&checksum);

        base58::encode_slice(buffer.as_ref())
    }

    pub fn sign(
        data: &[u8],
        secret_key: &SecretKey,
    ) -> Result<SerializedSignature, Secp256k1Error> {
        let hashed = Sha256Hash::hash(data).into_inner();

        Ok(Secp256k1::signing_only()
            .sign(&Message::from_slice(&hashed)?, secret_key)
            .serialize_der())
    }

    fn get_version_from_key(public_key: &PublicKey) -> [u8; 22] {
        let sh2 = Sha256Hash::hash(&public_key.serialize()).into_inner();
        let rp = Ripemd160Hash::hash(&sh2).into_inner();

        let mut ans = [0; 22];
        ans[0] = 0x0F;
        ans[1] = 0x02;
        for i in 2..22 {
            ans[i] = rp[i - 2];
        }

        ans
    }

    fn get_checksum_from_version(version: &[u8]) -> [u8; 4] {
        let h1 = Sha256Hash::hash(version).into_inner();
        let h2 = Sha256Hash::hash(&h1).into_inner();

        let mut ans = [0; 4];
        for i in 0..4 {
            ans[i] = h2[i];
        }

        ans
    }
}

#[derive(Debug, Clone)]
pub struct KeyPair {
    secret: SecretKey,
    pub public: PublicKey,
}

impl KeyPair {
    pub fn secret(&self) -> &SecretKey {
        &self.secret
    }
}

impl From<(SecretKey, PublicKey)> for KeyPair {
    fn from(other: (SecretKey, PublicKey)) -> Self {
        KeyPair {
            secret: other.0,
            public: other.1,
        }
    }
}

impl From<SecretKey> for KeyPair {
    fn from(other: SecretKey) -> Self {
        Cryptography::load_keypair(other)
    }
}

impl Deref for KeyPair {
    type Target = PublicKey;

    fn deref(&self) -> &Self::Target {
        &self.public
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bitcoin::hashes::hex::FromHex;

    const MY_PRIVATE_KEY: &str = "31eb31ecf1a640cd91e0a1105501f36235f8c7d51d67dcf74ccc968d74cb6b25";

    #[test]
    fn test_get_sin() {
        let key = SecretKey::from_slice(&Vec::<u8>::from_hex(MY_PRIVATE_KEY).unwrap()).unwrap();
        let keypair: KeyPair = key.into();

        let sin = Cryptography::get_sin_from_key(&keypair);
        assert_eq!(sin, "TfDnXWvj6bBhkduYiZnohg5qhtDu5VWohhw")
    }
}
