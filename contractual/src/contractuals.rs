use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use crate::Contractual;

#[derive(Serialize, Deserialize, Clone)]
pub struct Signed<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    pub message: T,
    pub signature: Signature,
}

impl<T> Signed<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone,
{
    pub fn new(message: T, signing_key: &SigningKey) -> Result<Self, String> {
        let mut serialized_message = Vec::new();
        ciborium::ser::into_writer(&message, &mut serialized_message)
            .map_err(|e| format!("Serialization error: {}", e))?;
        Ok(Self {
            message,
            signature: signing_key.sign(&serialized_message),
        })
    }

    pub fn verify(&self, verifying_key: &VerifyingKey) -> Result<(), String> {
        let mut serialized_message = Vec::new();
        ciborium::ser::into_writer(&self.message, &mut serialized_message)
            .map_err(|e| format!("Serialization error: {}", e))?;

        verifying_key.verify(&serialized_message, &self.signature)
            .map_err(|e| format!("Signature verification failed: {}", e))
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;

    #[test]
    fn test_signed() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        let message = String::from("Hello, World!");
        let signed = Signed::new(message, &signing_key).unwrap();
        assert!(signed.verify(&verifying_key).is_ok());
    }
}
