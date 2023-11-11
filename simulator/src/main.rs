use uuid::Uuid;
use sha1::{Sha1, Digest};

fn main() {
    let mut client = Client::new();
    let entry = client.new_entry(String::from(r#"{"a":1234}"#));
    client.new_entry(String::from(r#"{"a":4444}"#));
    client.new_entry(String::from(r#"{"a":9000}"#));

    client.set_index(0, entry.id, entry.hash);

    dbg!(client);
}

#[derive(Debug)]
struct Client {
    last_seen_index: i64,
    entries: Vec<Entry>,
}

impl Client {
    fn new() -> Self {
        Self {
            last_seen_index: -1,
            entries: Vec::new()
        }
    }

    fn new_entry(&mut self, data: String) -> Entry {
        let index = -1;
        let id = Uuid::new_v4();
        let prev = None;
        let hash = Checksum::calculate(&data);
        let entry = Entry { index, id, prev, hash, data };
        self.entries.push(entry.clone());
        entry
    }

    fn set_index(&mut self, index: i64, id: Uuid, hash: Checksum) {
        if index == self.last_seen_index {
            panic!(format!("index seen before: {}", index));
        } else if index < self.last_seen_index {
            panic!(format!("index below current last_seen_index: {} < {}", index, self.last_seen_index));
        }

        for entry in self.entries.iter_mut() {
            if entry.id == id && entry.hash == hash {
                entry.index = index;
            }
        }

        self.last_seen_index = index;
    }
}

struct Server {
    entries: Vec<Entry>,
}

#[derive(Debug, Clone)]
struct Entry {
    index: i64,
    id: Uuid,
    prev: Option<Checksum>,
    hash: Checksum,
    data: String
}

#[derive(PartialEq, Clone)]
struct Checksum(Vec<u8>);

impl Checksum {
    fn from_str(hash: &str) -> Result<Self, Error> {
        if hash.len() != 40 {
            Err(Error("wrong length"))
        } else if let Ok(value) = hex::decode(hash) {
            Ok(Self(value))
        } else {
            Err(Error("unable to decode hex"))
        }
    }

    fn calculate(data: &str) -> Self {
        let checksum = Sha1::digest(data.as_bytes());
        Self(checksum.to_vec())
    }

    fn as_string(&self) -> String {
        self.0.iter()
            .map(|x| format!("{:02x}", x))
            .collect()
    }
}

impl std::fmt::Debug for Checksum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

struct Error(&'static str);
