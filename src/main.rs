// use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use p256::ecdsa::{signature::Verifier, VerifyingKey};
// use p256::EncodedPoint;
use std::str;
// use std::str::FromStr;

fn main() {
    let public_key_hex: &str = "04934e4b30e93e7239bb8de910e80dee0908ea73f5f9247ab9c3b7fb7c8fd0a8688a868b97a54b57ab1aaafb119e8fac26267041f35b22050c1e593c35886258f8";
    // let message: &str = "cancu";

    let signature_hex: &str = "3044022075b060dde1b8dbeb50949db4ef234d8b0815c63392c970f8e6b7ebb60dc86b0c022045a55f7fa42c06246a27471509d29a53e0ca6d9bd28f9cd814bd361a11e20430";

    let signature_bytes: Vec<u8> = hex::decode(signature_hex).expect("Error decoding signature");

    // let public_key_bytes: Vec<u8> = hex::decode(public_key_hex).expect("Error decoding public key");

    println!("The bytes of the signature 👉 {:?}", signature_bytes);

    // // Verificar la firma
    // let is_valid_signature = verifying_key
    //     .verify(message.as_bytes(), &signature_bytes)
    //     .is_ok();

    // // Imprimir el resultado de la verificación
    // if is_valid_signature {
    //     println!("La firma es válida.");
    // } else {
    //     println!("La firma no es válida.");
    // }
}
