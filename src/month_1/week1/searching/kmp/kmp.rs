fn kmp_search_with_steps(text: &str, pattern: &str) -> Vec<usize> {
    println!("=== KMP Algorithm Step by Step ===");
    println!("Text: '{}'", text);
    println!("Pattern: '{}'", pattern);

    if pattern.is_empty() {
        println!("Empty pattern!");
        return vec![];
    }

    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    let n = text_bytes.len();
    let m = pattern_bytes.len();

    // Build LPS array with visualization
    let lps = build_lps_array_with_steps(pattern);

    println!("\n=== Starting Search ===");
    let mut matches = Vec::new();

    let mut i = 0; // Index for text
    let mut j = 0; // Index for pattern
    let mut step = 1;

    while i < n {
        println!(
            "\nStep {}: Comparing text[{}]='{}' with pattern[{}]='{}'",
            step, i, text_bytes[i] as char, j, pattern_bytes[j] as char
        );

        if pattern_bytes[j] == text_bytes[i] {
            println!("  ✓ Match! Moving both pointers");
            i += 1;
            j += 1;
        }

        if j == m {
            println!("  🎉 Complete match found at position {}", i - j);
            matches.push(i - j);
            j = lps[j - 1];
            if j > 0 {
                println!(
                    "  ↻ Using LPS[{}] = {} to continue searching",
                    m - 1,
                    lps[m - 1]
                );
            }
        } else if i < n && pattern_bytes[j] != text_bytes[i] {
            println!("  ✗ Mismatch!");
            if j != 0 {
                println!(
                    "  ↻ Using LPS[{}] = {} to skip characters",
                    j - 1,
                    lps[j - 1]
                );
                j = lps[j - 1];
            } else {
                println!("  → Moving text pointer");
                i += 1;
            }
        }

        step += 1;
    }

    println!("\n=== Search Complete ===");
    if matches.is_empty() {
        println!("No matches found.");
    } else {
        println!("Matches found at positions: {:?}", matches);
    }

    matches
}

/// Build LPS array with step-by-step explanation
fn build_lps_array_with_steps(pattern: &str) -> Vec<usize> {
    println!("\n=== Building LPS Array ===");
    let pattern_bytes = pattern.as_bytes();
    let m = pattern_bytes.len();
    let mut lps = vec![0; m];

    println!("Pattern: {}", pattern);
    print!("Index:   ");
    for i in 0..m {
        print!("{} ", i);
    }
    println!();

    let mut len = 0;
    let mut i = 1;

    while i < m {
        println!("\nStep: i={}, len={}", i, len);
        if pattern_bytes[i] == pattern_bytes[len] {
            len += 1;
            lps[i] = len;
            println!(
                "  Match! pattern[{}] == pattern[{}], LPS[{}] = {}",
                i,
                len - 1,
                i,
                len
            );
            i += 1;
        } else if len != 0 {
            println!(
                "  Mismatch! Moving len from {} to LPS[{}] = {}",
                len,
                len - 1,
                lps[len - 1]
            );
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            println!("  Mismatch at start, LPS[{}] = 0", i);
            i += 1;
        }
    }

    println!("\nFinal LPS array: {:?}", lps);
    lps
}

fn main() {
    let text = "ababcabcabababd";
    let pattern = "ababd";

    // Perform KMP search with step-by-step visualization
    let matches = kmp_search_with_steps(text, pattern);

    // Print final matches
    if matches.is_empty() {
        println!("No matches found.");
    } else {
        println!("Matches found at positions: {:?}", matches);
    }
}

fn test() {}
