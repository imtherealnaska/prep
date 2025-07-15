use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    sync::atomic::AtomicU64,
};

struct Codec {
    url_to_id: HashMap<String, u64>,
    id_to_url: HashMap<u64, String>,
    counter: AtomicU64,
    base_url: String,
}

impl Codec {
    fn new() -> Self {
        Self {
            url_to_id: HashMap::new(),
            id_to_url: HashMap::new(),
            counter: AtomicU64::new(1),
            base_url: "http://tinyurl.com/".to_string(),
        }
    }

    fn encode(&self, long_url: String) -> String {
        if let Some(&id) = self.url_to_id.get(&long_url) {
            return format!("{}{}", self.base_url, id);
        }

        let id = self
            .counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        self.url_to_id.insert(long_url.clone(), id);
        self.id_to_url.insert(id, long_url);

        format!("{}{}", self.base_url, id)
    }

    fn decode(&self, short_url: String) -> String {
        let id_str = short_url
            .strip_prefix(&self.base_url)
            .expect("Invalid short URL format");

        let id = id_str.parse().expect("Invalid id in short URL");

        self.id_to_url
            .get(&id)
            .cloned()
            .expect("Short URL not found")
    }
}

struct Codehash {
    code_to_url: HashMap<String, String>,
    base_url: String,
    char_set: Vec<char>,
}

impl Codehash {
    fn new() -> Self {
        let charset: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .collect();

        Self {
            code_to_url: HashMap::new(),
            base_url: "http://tinyurl.com/".to_string(),
            char_set,
        }
    }

    fn hash_to_base64(&self, hash: u64) -> String {
        let mut num = hash;
        if num == 0 {
            return self.char_set[0].to_string();
        }

        let mut result = String::new();
        let base = self.char_set.len() as u64;

        for _ in 0..6 {
            result.push(self.char_set[(num % base) as usize]);
            num /= base;
            if num == 0 {
                break;
            }
        }

        if result.is_empty() {
            result.push(self.char_set[0]);
        }

        result
    }

    // give a hash
    // collisions ?
    // diplicates ?
    fn encode(&mut self, long_url: String) -> String {
        let mut hasher = DefaultHasher::new();
        long_url.hash(&mut hasher);
        let hash = hasher.finish();

        let mut code = self.hash_to_base64(hash);
        let mut counter = 0;
        let mut og_code = code;

        while self.code_to_url.contains_key(&code) {
            if self.code_to_url[&code] == long_url {
                return format!("{}{}", self.base_url, code);
            }

            // collision
            counter += 1;
            code = format!("{}{}", og_code, counter);
        }

        self.code_to_url.insert(code.clone(), long_url);
        format!("{}{}", self.base_url, code)
    }

    fn decode(&mut self, short_url: String) -> String {
        let code = short_url
            .strip_prefix(&self.base_url)
            .expect("Invalid  short URL");

        self.code_to_url
            .get(&code)
            .cloned()
            .expect("Short url not found")
    }
}
