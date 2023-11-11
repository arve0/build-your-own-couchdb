use sha1::{Digest, Sha1};

fn main() {
    // create a Sha1 object
    let mut hasher = Sha1::new();

    // process input message
    hasher.input("d1a26b7582202a5af361");
    hasher.input("853edf0e179480988df8");
    hasher.input(r#"{"a":4321,"b":"qwerty"}"#);

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 20]
    let result = hasher.result();
    println!("{:x}", &result);

    let data: Vec<u8> = br#"d1a26b7582202a5af361853edf0e179480988df8{"a":4321,"b":"qwerty"}"#.to_vec();
    println!("{:x}", Sha1::digest(&data));
}

