use casper_types::{bytesrepr::{ToBytes, FromBytes, Bytes}, CLType};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

fn main() {
    let bytes = hex::decode("1600000010000000030000000000000001000000020000000e04").unwrap();
    let bytes: Vec<u8> = vec![22,0,0,0,16,0,0,0,3,0,0,0,0,0,0,0,1,0,0,0,2,0,0,0,14,4];
    let (bytes, remainder) = Bytes::from_bytes(&bytes).unwrap();
    let (bytes, remainder) = Bytes::from_bytes(&bytes).unwrap();
    println!("{:?}", remainder);
    let (ty, rem) = CLType::from_bytes(&remainder).unwrap();
    println!("{:?}", ty);
    let (list, remainder) = Vec::<u32>::from_bytes(&bytes).unwrap();
    println!("{:?}", list);

    println!("{:?}", u32::from_vec(vec![255, 255, 255, 255]));
    println!("{:?}", u32::MAX);
}

fn test_key58_decode() {
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
