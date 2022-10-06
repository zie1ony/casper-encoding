use casper_types::bytesrepr::{ToBytes, Bytes};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

fn main() {
    let key = "PD4bbfLSeqWrVrqZT1LsHzboAf1KdnC1PxDrSH8xSgYrJAgdtTbNQ";
    let key58 = bs58::decode(key).into_vec().unwrap();
    println!("bs58: {:?}", &key58);
    let key58: Bytes = key58.into();
    println!("Bytes: {:?}", hex::encode(&key58));
    let key = to_dictionary_item_key(&key58);
    println!("Final: {:?}", key);
}

fn to_dictionary_item_key<T: ToBytes>(key: &T) -> String {
    let preimage = key.to_bytes().unwrap();
    println!("Preimage: {:?}", &preimage);
    let hash = blake2b(preimage);
    hex::encode(hash)
}

fn blake2b<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let mut result = [0; 32];
    let mut hasher = VarBlake2b::new(32).expect("should create hasher");

    hasher.update(data);
    hasher.finalize_variable(|slice| {
        result.copy_from_slice(slice);
    });
    result
}
