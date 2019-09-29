use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;


fn main() {
    ba1a();
    ba1b();
}

fn ba1a() {
    print!("Problem BA1A: ");
    let text = String::from("ACAGATCACGACAGATCAATCAGATCAATCAGATCATTCAGATCAGTATCACGCAGATCAAACAGATCACCAACAGATCAGCAATCAGATCACAGATCACCAGATCATCAGATCATGCAGATCATCCAGATCATCATCAGATCACGCCAGATCACAGATCATGGGAATTCAGATCACAGATCATCCAGATCACAGATCACAGATCAGCGCAGATCATGTCCACGTAAAACACCCTCAGATCACAGATCAATCGCAGATCAGGAACCAGATCATGCCTTCAGATCACGAATCAGATCACGTGCAGATCACAGATCAACAGATCACAGATCACAGATCAGGTCAGATCATACAGATCAGCAGATCACAGATCAAGCGCAGATCATCAGATCAAGAATTGGTTTCCAGATCACCAGATCACAGATCATCCAGATCACAGATCAAACTGCAGATCATTTTAACAGATCATCAGATCAATGCAGATCAACAGATCAAGATAACAGATCACAGATCATACAGATCAGACAGATCATAAGCTTGCAGATCACAGATCAAGTCTCAGATCACAGATCATCAGATCACAGATCAATGGACAGATCATGCGTCAGATCACAGATCAAAGCTGAATGCGGACAGATCACAGATCACAGATCAGCAGATCACAGATCACACATCAGATCAGCAGATCAGGTAACAGATCAGCAGATCAACAGATCACCTCGCCAGATCAGACAGATCATACAGATCATTCAGATCACCAGATCACGCAGATCACAGATCAGCAGATCATTCAGATCACAGATCACAGATCATACAGATCACGCCAGATCACAGATCATCCAGATCATAACCAACAGATCAGCAGATCAGTAGACCAGATCATACAGATCATGCCTGTCAGATCAAGTCACAGATCATGCAGCTCAGATCACAGATCAATACAGATCATTCAGATCAAACCAGATCACAGATCATAAGATGCACAGATCAACGCAGATCAGA");
    let pattern = String::from("CAGATCACA");
    println!("{}", kmer_count(&text, &pattern))
}

fn ba1b() {
    print!("Problem BA1B: ");
    let text = String::from("GAGTCTATACTCTCGGCGTGTTATTAGTATTGTAACGTTTAGTATTACTCTCGTTAGTATTGTAACGTGCGTGTTAGAGTCTATTTAGTATTGAGTCTATTTAGTATTGTAACGTGCGTGTTAGAGTCTATACTCTCGGAGTCTATGAGTCTATGTAACGTTTAGTATTACTCTCGGTAACGTGAGTCTATGTAACGTGAGTCTATGAGTCTATGTAACGTTTAGTATTGTAACGTTTAGTATTTTAGTATTGCGTGTTAGCGTGTTAGAGTCTATGTAACGTGCGTGTTATTAGTATTTTAGTATTGCGTGTTAGAGTCTATGTAACGTACTCTCGACTCTCGTTAGTATTACTCTCGGAGTCTATACTCTCGTTAGTATTGTAACGTGTAACGTTTAGTATTGAGTCTATACTCTCGTTAGTATTGCGTGTTAGTAACGTGCGTGTTAACTCTCGTTAGTATTGCGTGTTAACTCTCGGAGTCTATACTCTCGTTAGTATTTTAGTATTTTAGTATTGAGTCTATTTAGTATTGTAACGTACTCTCGGAGTCTATGAGTCTATTTAGTATTACTCTCGGAGTCTATTTAGTATTGTAACGTGCGTGTTAGTAACGTGCGTGTTAGCGTGTTAACTCTCGGAGTCTATGTAACGTACTCTCGGCGTGTTATTAGTATTACTCTCGGAGTCTATGCGTGTTAACTCTCGGCGTGTTATTAGTATTTTAGTATTGAGTCTATGCGTGTTAACTCTCGGTAACGTGAGTCTATGTAACGTGAGTCTATGCGTGTTATTAGTATTGCGTGTTAGAGTCTATGTAACGT");
    let k = 12;
    let kmers = most_frequent_kmers(&text, k);
    println!("{:?}", kmers);
}

fn most_frequent_kmers(text: &String, k: usize) -> Vec<&str> {
    let hash = most_frequent_kmer_hash(text, k);
    let inverted_hash: HashMap<usize, Vec<&str>> = inefficient_invert_hash(hash);

    let mut max_key: &usize = &0;
    for key in inverted_hash.keys() {
        if key > max_key {
            max_key = key;
        }
    }

    let kmers = inverted_hash[max_key].clone();
    kmers
}

// fn invert_hash<'a, K: Eq + Hash, V: Eq + Hash>(hash: &'a HashMap<K, V>) -> HashMap <&'a V, &'a Vec<K>> {
//     let mut inverted: HashMap<&V, &Vec<K>> = HashMap::new();
//     let hash = hash.iter();

//     for (key, value) in hash {
//         let mut vec = match inverted.get_mut(value) {
//             Some(vec) => vec,
//             None => &Vec::new(),
//         };
//         let key_clone = key.clone();
//         vec.push(key_clone);
//         inverted.insert(value, &vec);
//     }
//     inverted
// }

fn inefficient_invert_hash<K: Eq + Hash + Clone + Debug, V: Eq + Hash + Clone>(hash: HashMap<K, V>) -> HashMap<V, Vec<K>> {
    let mut inverted: HashMap<V, Vec<K>> = HashMap::new();
    let hash2 = hash.clone();
    let hash = hash.iter();
    for (_key, value) in hash {
        let v = Vec::new();
        let new_val = value.clone();
        inverted.insert(new_val, v);
    }

    let hash2 = hash2.iter();
    for (key, value) in hash2 {
        match inverted.get_mut(value) {
            Some(vec) => {
                let new_key = key.clone();
                vec.push(new_key)
            },
            None => {},
        }
    }
    inverted
}

fn most_frequent_kmer_hash(text: &String, k: usize) -> HashMap<&str, usize> {
    let mut freq_hash = HashMap::new();
    let limit = text.len() - k;
    for start_pos in 0..=limit {
        let substring = &text[start_pos..(start_pos + k)];
        let new_count: usize = match freq_hash.get(substring) {
            Some(num) => num + 1,
            None => 1,
        };
        freq_hash.insert(substring, new_count);
    }
    freq_hash
}

fn kmer_count(text: &String, pattern: &String) -> usize {
    let mut count = 0;
    let limit = text.len() - pattern.len();
    for start_pos in 0..=limit {
        let substring = &text[start_pos..(start_pos + pattern.len())];
        if substring == &pattern[..] {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_frequent_kmers() {
        struct TestCase {
            text: String,
            k: usize,
            most_freq: Vec<String>
        }
        let cases = [
            TestCase {
                text: String::from("ACGTTGCATGTCGCATGATGCATGAGAGCT"),
                k: 4,
                most_freq: vec![String::from("CATG"), String::from("GCAT")]
            },
            TestCase {
                text: String::from("TGGTAGCGACGTTGGTCCCGCCGCTTGAGAATCTGGATGAACATAAGCTCCCACTTGGCTTATTCAGAGAACTGGTCAACACTTGTCTCTCCCAGCCAGGTCTGACCACCGGGCAACTTTTAGAGCACTATCGTGGTACAAATAATGCTGCCAC"),
                k: 3,
                most_freq: vec![String::from("TGG")]
            },
            TestCase {
                text: String::from("CAGTGGCAGATGACATTTTGCTGGTCGACTGGTTACAACAACGCCTGGGGCTTTTGAGCAACGAGACTTTTCAATGTTGCACCGTTTGCTGCATGATATTGAAAACAATATCACCAAATAAATAACGCCTTAGTAAGTAGCTTTT"),
                k: 4,
                most_freq: vec![String::from("TTTT")]
            },
            TestCase {
                text: String::from("ATACAATTACAGTCTGGAACCGGATGAACTGGCCGCAGGTTAACAACAGAGTTGCCAGGCACTGCCGCTGACCAGCAACAACAACAATGACTTTGACGCGAAGGGGATGGCATGAGCGAACTGATCGTCAGCCGTCAGCAACGAGTATTGTTGCTGACCCTTAACAATCCCGCCGCACGTAATGCGCTAACTAATGCCCTGCTG"),
                k: 5,
                most_freq: vec![String::from("AACAA")]
            },
            TestCase {
                text: String::from("CCAGCGGGGGTTGATGCTCTGGGGGTCACAAGATTGCATTTTTATGGGGTTGCAAAAATGTTTTTTACGGCAGATTCATTTAAAATGCCCACTGGCTGGAGACATAGCCCGGATGCGCGTCTTTTACAACGTATTGCGGGGTAAAATCGTAGATGTTTTAAAATAGGCGTAAC"),
                k: 5,
                most_freq: vec![String::from("AAAAT"), String::from("GGGGT"), String::from("TTTTA")]
            }
        ];

        for case in cases.iter() {
            let test_most_freq_vec: Vec<&str> = case.most_freq.iter().map(|x| &x[..]).collect();
            let mut most_freq_vec = most_frequent_kmers(&case.text, case.k);
            most_freq_vec.sort_unstable();
            assert_eq!(most_freq_vec, test_most_freq_vec);
        }
    }

    // #[test]
    // fn test_most_frequent_kmer_hash() {
    //     struct TestCase {
    //         text: String,
    //         k: usize,
    //         freq_hash: HashMap<String, usize>
    //     }
    //     let cases = [
    //         TestCase {
    //             text: String::from("ACGTTGCATGTCGCATGATGCATGAGAGCT"),
    //             k: 4,
    //             freq_hash: HashMap::new()
    //         },
    //     ];

    //     let cases = cases.iter();

    //     fn build_new_hash<T>(hash: HashMap<String,T>) -> HashMap<&str,T> {
    //         let hash = hash.iter();
    //         let mut new_hash = HashMap::new();
    //         for (key, value) in hash {
    //             new_hash.insert(&key[..], value)
    //         }
    //         new_hash
    //     }

    //     for case in cases {
    //         let hash = build_new_hash(case.freq_hash);
    //         assert_eq!(most_frequent_kmer_hash(&case.text, case.k), hash);
    //     }
    // }

    #[test]
    fn test_most_frequent_kmer_hash() {
        let text = String::from("ACGTTGCATGTCGCATGATGCATGAGAGCT");
        let k = 4;

        let hash = most_frequent_kmer_hash(&text, k);
        let gcat_count: Option<&usize> = Some(&3);
        assert_eq!(hash.get("GCAT"), gcat_count);
        let gatg_count: Option<&usize> = Some(&1);
        assert_eq!(hash.get("GATG"), gatg_count);
        assert_eq!(hash.get("ZZZZ"), None);
    }

    // #[test]
    // fn test_invert_hash() {
    //     let hash: HashMap<_, _> = vec![
    //         (String::from("key1"), String::from("value")),
    //         (String::from("key2"), String::from("value"))
    //     ].iter().cloned().collect();

    //     let value = String::from("value");
    //     let key1 = String::from("key1");
    //     let key2 = String::from("key2");
    //     let inverted_hash: HashMap<_, _> = vec![
    //         (&value, &vec![key1, key2])
    //     ].iter().cloned().collect();

    //     assert_eq!(invert_hash(&hash), inverted_hash);
    // }

    #[test]
    fn test_inefficient_invert_hash() {
        let hash: HashMap<_, _> = vec![
            (String::from("key1"), String::from("value")),
            (String::from("key2"), String::from("value"))
        ].iter().cloned().collect();

        let value = String::from("value");
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let test_inverted_hash: HashMap<_, _> = vec![
            (value, vec![key1, key2])
        ].iter().cloned().collect();

        let mut inverted_hash = inefficient_invert_hash(hash);
        // We care that the vector values of the inverted hash 
        // contain the correct values but are agnostic to the
        // order- we sort here for deterministic testing.
        for (_key, value) in inverted_hash.iter_mut() {
            value.sort_unstable();
        }

        assert_eq!(inverted_hash, test_inverted_hash);
    }

    #[test]
    fn test_kmer_count() {
        #[derive(Debug)]
        struct TestCase<'a> {
            text: &'a String,
            pattern: &'a String,
            count: usize
        }
        let cases = [
            TestCase {
                text: &String::from("GCGCG"),
                pattern: &String::from("GCG"),
                count: 2
            },
            TestCase {
                text: &String::from("ACGTACGTACG"),
                pattern: &String::from("CG"),
                count: 3
            },
            TestCase {
                text: &String::from("AAAGAGTGTCTGATAGCAGCTTCTGAACTGGTTACCTGCCGTGAGTAAATTAAATTTTATTGACTTAGGTCACTAAATACTTTAACCAATATAGGCATAGCGCACAGACAGATAATAATTACAGAGTACACAACATCCA"),
                pattern: &String::from("AAA"),
                count: 4
            },
            TestCase {
                text: &String::from("AGCGTGCCGAAATATGCCGCCAGACCTGCTGCGGTGGCCTCGCCGACTTCACGGATGCCAAGTGCATAGAGGAAGCGAGCAAAGGTGGTTTCTTTCGCTTTATCCAGCGCGTTAACCACGTTCTGTGCCGACTTT"),
                pattern: &String::from("TTT"),
                count: 4
            },
            TestCase {
                text: &String::from("GGACTTACTGACGTACG"),
                pattern: &String::from("ACT"),
                count: 2
            }
        ];

        for case in cases.iter() {
            println!("{:?}", case);
            assert_eq!(kmer_count(case.text, case.pattern), case.count)
        }
    }
}