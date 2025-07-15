pub struct KMP {
    lps: Vec<usize>,
    pattern: Vec<u8>,
}

impl KMP {
    pub fn new(pattern: &str) -> Self {
        let pattern = pattern.as_bytes().to_vec();
        let lps = Self::compute_lps(&pattern);
        KMP { lps, pattern }
    }

    pub fn compute_lps(pattern: &[u8]) -> Vec<usize> {
        let mut lps = vec![0; pattern.len()];
        let mut len = 0;
        let mut i = 1;

        while i < pattern.len() {
            if pattern[i] == pattern[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else {
                if len != 0 {
                    len = lps[len - 1];
                } else {
                    lps[i] = 0;
                    i += 1;
                }
            }
        }
        lps
    }

    pub fn find_matches(&self, text: &str) -> Vec<usize> {
        let text = text.as_bytes();
        let mut matches = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < text.len() {
            if self.pattern[j] == text[i] {
                i += 1;
                j += 1;
            }

            if j == self.pattern.len() {
                matches.push(i - j);
                j = self.lps[j - 1];
            } else if i < text.len() && self.pattern[j] != text[i] {
                if j != 0 {
                    j = self.lps[j - 1];
                } else {
                    i += 1;
                }
            }
        }
        matches
    }
}
