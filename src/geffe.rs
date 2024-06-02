use std::time::Instant;
use std::vec::Vec;

#[allow(dead_code)]
const EXAMPLE_STRING: &str = "00111100011010100111011000000101011111001011010000101001001111010011101111110010011010000011011110100001011001001000010010111010001110000100010100110111100011010110100111100111001110000010111011111111010110010110011010101110110111011010000101000010010000010111010100010111010010001101010001010111000010111100011110100011100011011010010011001001011100001101100100110000011110101100100001000100000101011111111101010101110011001000111001110110110001110001000010101001011010010101011011110000111101101000111001111101111011010101100110001001010000010010001100100001110001010100010011011111100111011111010110000001111000010000111110101111010111111011101001011010000100001010101101000100010011101011111111101011000000001110011011101100110110001010010011001101100110100011011000111000111110100110100010101010100011111010111010110101100011000000010000010111000101100001010111001000000100010100101011000100110110101100111010110111001110001001001111010011011110010101011100100110110111110111110111010110100010110111010110101011101011100100010101011000100101111011110101010100101101010111011111100001101111010100011011011011111100010100110111100110111111011011101010011011111110100101001100111010101101000010111010010000111001110001000001110011010111111111101111111000110011001010011110001111100011110100110100001010001111000100110011000101000011100001000001110001001100011000010111111010001100101110011110001111101101101001010010011100101100111110010100101100110001111101011001101111001011001001001110000110000110000001001101101100110001000101010010001110000110010001000001000110101000011100100101101011101101111101000100110000100101000110001101010011100101011110001111000100111110010111010000111101010001100101011111100110110011110001110000100101110101110011100110111101011100111101001000101000001100100001101110000101101011011001010101100110110110111000111011101111001110010010000010111010001111000000011001010010100000110001100000010101000111110101000100001100100101000011000010100100001110110011101101111010001100100101001011011111001110001011010010110011";

#[allow(dead_code)]
const SIGMA_STRING: &str = "01101110110010000110100111111011101110110010001010000011111100101001011111001001001100001100101010011011110010010111110100011000010000010110010000001001110110101000111111111001100010111000000001000001000110011111001100000000101010011110101110010110111000001000000111111011000011010000101110100000010000000111101110010111100100111111000110111101101110001110101010110111000101011100011011111010101101111001000000100100000000110011011100011001001000000101110010111010011000001010101110011010100011100000110000011000111110110001000100001011111010000001010000011011000010001010111001101010100011100001111001110101011100111101101010010101001010101110100101100110101011111011101001001011000110010000010011111000000000010001100110100000101000000010010110000011110100110110001010010101110111000001011001100100011100001111011010101011000011111101001101001000111011101100000011001111001111101010101111010010001011010000110000110010101011110001011101001111010110000010001000111010010010110111001001101111110111000110000110010000101100100010001001011101001011101101111101111011110001110111101001101000011101111000000011111111111111001101101000100110101000110111001000000111101011011000000100110011100001010000011101010001110111011001100100010000100111110110000110110011100111110000111011001111010111110001110110111110001000111000010010100000101010101010001111000110011011011110010110100110100000000000110000111000011011110100000001000000001100101011100000000110001000110100110001011010110101010001100101111011010011111010010110100101110100100101101001111001101010111000011101101011001001101100011001111110001000100100010011000001000101100111111000001110010010100110100001110110010111101111100101011010011101000100001111110010101100101011101110001000001100110110101100010011010110001000010110110001011011101001010101000000001101000101011001100110101001101111100101100101111111010101010000110001101101001000100111101100100010111000101110100101110000001111110001101100011110110001110100000011000001100110111001110011011101111100011001110000000111010110111001011110"; 

const STRING_LEN: usize = 2048;

struct LFSR {
    state: u32,
    highest_bit: u8,
    taps: u32,
}

impl LFSR {
    fn new(taps: u32, degree: u8) -> LFSR {
        let highest_bit = degree - 1;
        LFSR {
            state: 0,
            highest_bit,
            taps,
        }
    }

    fn generate(&mut self, seed: u32, length: u64) -> Vec<u8> {
        self.state = seed;
        let mut output = vec![0u8; length as usize];

        for i in 0..length {
            output[i as usize] = (self.state & 1) as u8;
            self.state = (self.state >> 1)
                ^ (((self.state & self.taps).count_ones() & 1) << self.highest_bit);
        }

        output
    }
}

struct Geffe {
    lfsr1: LFSR,
    lfsr2: LFSR,
    lfsr3: LFSR,
}

impl Geffe {
    fn new(lfsr1: LFSR, lfsr2: LFSR, lfsr3: LFSR) -> Geffe {
        Geffe { lfsr1, lfsr2, lfsr3 }
    }

    fn generate(&mut self, seed1: u32, seed2: u32, seed3: u32, length: usize) -> Vec<u8> {
        let mut result = vec![0u8; length];

        let seq1 = self.lfsr1.generate(seed1, length as u64);
        let seq2 = self.lfsr2.generate(seed2, length as u64);
        let control = self.lfsr3.generate(seed3, length as u64);

        for i in 0..length {
            result[i] = if control[i] == 1 { seq1[i] } else { seq2[i] };
        }

        result
    }
}

fn find_candidates(
    lfsr: &mut LFSR,
    target_seq: &[u8],
    required_len: usize,
    threshold: usize,
    degree: u8,
) -> Vec<(u32, usize)> {
    let cycle_len = (1u64 << degree) + required_len as u64;
    let mut current_candidate = 1u32;
    let generated_seq = lfsr.generate(current_candidate, cycle_len);

    let mut candidates = Vec::new();
    for j in 0..(1u64 << degree) {
        let mut discrepancy = 0;
        for i in 0..required_len {
            discrepancy += (generated_seq[j as usize + i] ^ target_seq[i]) as usize;
        }

        if discrepancy < threshold {
            candidates.push((current_candidate, discrepancy));
        }

        current_candidate = (current_candidate >> 1)
            ^ (((generated_seq[degree as usize + j as usize] as u32) << (degree - 1)) as u32);
    }

    candidates
}

fn find_best_candidate(candidates: &[(u32, usize)], required_len: usize) -> u32 {
    let mut best_candidate = candidates[0].0;
    let mut min_deviation = candidates[0].1 as f32 - 0.25 * required_len as f32;
    for &(candidate, discrepancy) in candidates {
        let deviation = discrepancy as f32 - 0.25 * required_len as f32;
        if deviation < min_deviation {
            min_deviation = deviation;
            best_candidate = candidate;
        }
    }

    best_candidate
}

fn find_lfsr3_candidate(
    lfsr3: &mut LFSR,
    lfsr1: &mut LFSR,
    lfsr2: &mut LFSR,
    target_seq: &[u8],
    lfsr1_candidate: u32,
    lfsr2_candidate: u32,
    n: usize,
    degree: u8,
) -> u32 {
    let cycle_len = (1u64 << degree) + n as u64;
    let mut current_candidate = 1u32;
    let lfsr3_seq = lfsr3.generate(current_candidate, cycle_len);

    let lfsr1_seq = lfsr1.generate(lfsr1_candidate, n as u64);
    let lfsr2_seq = lfsr2.generate(lfsr2_candidate, n as u64);

    for j in 0..(1u64 << degree) {
        let mut match_found = true;
        for i in 0..n {
            if ((lfsr3_seq[j as usize + i] & lfsr1_seq[i])
                ^ ((1 ^ lfsr3_seq[j as usize + i]) & lfsr2_seq[i]))
                != target_seq[i]
            {
                match_found = false;
                break;
            }
        }

        if match_found {
            return current_candidate;
        }

        current_candidate = (current_candidate >> 1)
            ^ (((lfsr3_seq[degree as usize + j as usize] as u32) << (degree - 1)) as u32);
    }

    0
}

fn main() {
    let start = Instant::now();
    // dummy 
    let lfsr1_taps = (1 << 3) ^ 1;
    let lfsr1_degree = 25;
    let lfsr2_taps = (1 << 6) ^ (1 << 2) ^ (1 << 1) ^ 1;
    let lfsr2_degree = 26;
    let lfsr3_taps = (1 << 5) ^ (1 << 2) ^ (1 << 1) ^ 1;
    let lfsr3_degree = 27;

    let n = EXAMPLE_STRING.len();
    let target_seq: Vec<u8> = EXAMPLE_STRING.chars().map(|c| c as u8 - 48).collect();

    let lfsr1_required_len = 222;
    let lfsr1_threshold = 71;
    let lfsr2_required_len = 229;
    let lfsr2_threshold = 74;

    // sigma 
    // let lfsr1_taps = (1 << 6) ^ (1 << 5) ^ (1 << 1) ^ 1;
    // let lfsr1_degree = 30;
    // let lfsr2_taps = (1 << 3) ^ 1;
    // let lfsr2_degree = 31;
    // let lfsr3_taps = (1 << 7) ^ (1 << 5) ^ (1 << 3) ^ (1 << 2) ^ (1 << 1) ^ 1;
    // let lfsr3_degree = 32;

    // let n = SIGMA_STRING.len();
    // let target_seq: Vec<u8> = SIGMA_STRING.chars().map(|c| c as u8 - 48).collect();

    // let lfsr1_required_len = 258;
    // let lfsr1_threshold = 83;
    // let lfsr2_required_len = 265;
    // let lfsr2_threshold = 81;

    let mut lfsr1 = LFSR::new(lfsr1_taps, lfsr1_degree);
    let mut lfsr2 = LFSR::new(lfsr2_taps, lfsr2_degree);
    let mut lfsr3 = LFSR::new(lfsr3_taps, lfsr3_degree);

    let lfsr1_candidates = find_candidates(&mut lfsr1, &target_seq, lfsr1_required_len, lfsr1_threshold, lfsr1_degree);
    println!("LFSR1 finished with: {} candidates", lfsr1_candidates.len());

    let lfsr2_candidates = find_candidates(&mut lfsr2, &target_seq, lfsr2_required_len, lfsr2_threshold, lfsr2_degree);
    println!("LFSR2 finished with: {} candidates", lfsr2_candidates.len());

    let lfsr1_candidate = find_best_candidate(&lfsr1_candidates, lfsr1_required_len);
    let lfsr2_candidate = find_best_candidate(&lfsr2_candidates, lfsr2_required_len);
    let lfsr3_candidate = find_lfsr3_candidate(
        &mut lfsr3, &mut lfsr1, &mut lfsr2, &target_seq, lfsr1_candidate, lfsr2_candidate, n, lfsr3_degree
    );

    println!("LFSR3 finished");

    println!(
        "\nLFSR1 candidate: {:10} {:032b}",
        lfsr1_candidate, lfsr1_candidate
    );
    println!(
        "\nLFSR2 candidate: {:10} {:032b}",
        lfsr2_candidate, lfsr2_candidate
    );
    println!(
        "\nLFSR3 candidate: {:10} {:032b}",
        lfsr3_candidate, lfsr3_candidate
    );

    println!("Testing results...");
    let mut generator = Geffe::new(lfsr1, lfsr2, lfsr3);

    let test_seq = generator.generate(lfsr1_candidate, lfsr2_candidate, lfsr3_candidate, STRING_LEN);

    println!("Generated sequence: ");
    for c in test_seq {
        print!("{}", c);
    }
    println!();

    println!("Expected sequence: ");
    for c in &target_seq {
        print!("{}", c);
    }
    println!();

    let duration = start.elapsed();
    println!("Execution time: {} seconds", duration.as_secs());
}