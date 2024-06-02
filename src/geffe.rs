use std::time::Instant;
use std::vec::Vec;
use structopt::StructOpt;

#[allow(dead_code)]
const BETA_STRING: &str = "01111110100111111111001110000000001010101010110110100100010100011100110110101100000010101010011101011100010001101011100110011100111011101001001010100001111011010111100001000110001000010110000010011100101111100111101101011111111011110100000000100100010000100000101101101111100010100001110000011111010001101101000101111000111010110000101011101111010010101000111000011110010100100101101011011011100010010001100001100000001100111100101111100000001000110100100001110000001011000101100001111000011001101101010110101111100000101101100111110101111010110000001110001001010011010010100010000001110110100111100101110010001000000101000001001001111101000111101100110110110011010100001100011000001000011101000101000001111111101110010001110101011100100011000000010111001011101010010000110010111100101100101100101100100110011011010101101111110111110001110011101010101100111010111010101110100011000000010110010001010011100101110011110001111001010111010101110101011100011001111100101001001000101110001110011000100100001100010010001001000001110010000010100011101111101010001110111001011101111101011000010001010001010101011101001100101111000101010010110110000110100001000000000011010101010100110000100101011001100000000010100101010011111001010111010010000010010010111101100101011001110111001101000010000101001011001100100111000111111111111010011100100100001101111110100111011010000110101101101111101110111110100100011111010101011011001010110010100101001110111111010111101100001011111001000101000000011100000001001110111010100000000110010110111100100101010110111101000001001001110011101011101101110011011010101110111001100110001011000101111010101011010000100111010011110100001001101111110100011010000010001010100000010001100011110000101010000010111000011101100101011110111111011010100100000001001000111011100110110110010100010111111101011111101111111010011101011000000001000110110100110111100011111001001101000110000100101001110010111101110101001100100000100101100100010001101010011011111101010111011000001111101111110010001000011001101010100111111111001010001110010011";

#[allow(dead_code)]
const SIGMA_STRING: &str = "01111111101101001101101111011110101111011000010010001100000110011011001110000001011110100000001000000100011110001000111110000110010010001111101110100000110001110001110010011100001110011001001111110010000011010110001001001000000011011000010000111101101101010101000111011111111110001111011110110110000001000011100000010110000110001101101100111011000011001000010101111111101001000100000111111100010100111000110110001011000000100101010010111001101111001011001111101100011011110011010001111100001010110001111110010011011010011111011010100011111001100100100001100001010110010001100101110011010100110010100010000111100100011011101000110001011001000000101010101101010101111110010011110110110101111000000101001000000001101100101000001111011000011100001001000000000000101100110110101000101110001101000111110100111000000101011010010111100001000100000101000011001101110011010100100100011101010010110011110001000010010111100101101110001000011000000000100000101010000111011100000100100100111101111001001110111001111011000111100100111110001101000011110101111001000111111111101110101101010011101000110011101111000011000111100111000111000000010101111011000101010110110011101101001001010101100101011101110010010111000100000100000001101000011101001010111000111101100101001001111100010011001010101100100010110101000110010011100111101001100110011000000101000110100110111010100110000010000001010110010100001011111100100011100110100010011010100001010111100000100001001000010001100001010001101011101010101011101100100010110001011101101011000101011101011110011001010110111011000100101110111000001001001000101000101001000010011000111011011110000011100101011011111100110001010110110000111011110111110010111100000000111101000100100111101001010001001010111000000011000011001011100100001100100111010011001010001100010110010111101101011101111000000111110101000110000000010100101110011010101000011101011000001110111101110001110011000101001100101101110001010001001101011111110010000100011001001111111000011001010110111111011010001101100100011010010100011110000000000111100100100111"; 

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

        current_candidate = (current_candidate >> 1) ^ (((lfsr3_seq[degree as usize + j as usize] as u32) << (degree - 1)) as u32);
    }

    0
}

fn run_beta() {
    let lfsr1_taps = (1 << 3) ^ 1;
    let lfsr1_degree = 25;
    let lfsr2_taps = (1 << 6) ^ (1 << 2) ^ (1 << 1) ^ 1;
    let lfsr2_degree = 26;
    let lfsr3_taps = (1 << 5) ^ (1 << 2) ^ (1 << 1) ^ 1;
    let lfsr3_degree = 27;

    let n = BETA_STRING.len();
    let target_seq: Vec<u8> = BETA_STRING.chars().map(|c| c as u8 - 48).collect();

    let lfsr1_required_len = 222;
    let lfsr1_threshold = 71;
    let lfsr2_required_len = 229;
    let lfsr2_threshold = 74;

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

    println!("Comparing...");
    let mut generator = Geffe::new(lfsr1, lfsr2, lfsr3);

    let test_seq = generator.generate(lfsr1_candidate, lfsr2_candidate, lfsr3_candidate, STRING_LEN);

    println!("Generated sequence: ");
    for c in &test_seq {
        print!("{}", c);
    }
    println!();

    println!("Expected sequence: ");
    for c in &target_seq {
        print!("{}", c);
    }
    println!();

    if test_seq == target_seq {
        println!("The generated sequence matches the target sequence.");
    } else {
        println!("The generated sequence does not match the target sequence.");
    }

    println!();
}

fn run_sigma() {
    let lfsr1_taps = (1 << 6) ^ (1 << 5) ^ (1 << 1) ^ 1;
    let lfsr1_degree = 30;
    let lfsr2_taps = (1 << 3) ^ 1;
    let lfsr2_degree = 31;
    let lfsr3_taps = (1 << 7) ^ (1 << 5) ^ (1 << 3) ^ (1 << 2) ^ (1 << 1) ^ 1;
    let lfsr3_degree = 32;

    let n = SIGMA_STRING.len();
    let target_seq: Vec<u8> = SIGMA_STRING.chars().map(|c| c as u8 - 48).collect();

    let lfsr1_required_len = 258;
    let lfsr1_threshold = 83;
    let lfsr2_required_len = 265;
    let lfsr2_threshold = 81;

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

    println!("Comparing...");
    let mut generator = Geffe::new(lfsr1, lfsr2, lfsr3);

    let test_seq = generator.generate(lfsr1_candidate, lfsr2_candidate, lfsr3_candidate, STRING_LEN);

    println!("Generated sequence: ");
    for c in &test_seq {
        print!("{}", c);
    }
    println!();

    println!("Expected sequence: ");
    for c in &target_seq {
        print!("{}", c);
    }
    println!();

    if test_seq == target_seq {
        println!("The generated sequence matches the target sequence.");
    } else {
        println!("The generated sequence does not match the target sequence.");
    }

    println!();
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(long)]
    dummy: bool,
}

fn main() {
    let args = Cli::from_args();

    let start = Instant::now();

    if args.dummy {
        run_beta();
    } else {
        run_sigma();
    }

    let duration = start.elapsed();
    println!("Execution time: {} seconds", duration.as_secs());
}