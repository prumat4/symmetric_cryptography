use std::time::Instant;
use std::vec::Vec;
use structopt::StructOpt;

#[allow(dead_code)]
const BETA_STRING: &str = "01111110100111111111001110000000001010101010110110100100010100011100110110101100000010101010011101011100010001101011100110011100111011101001001010100001111011010111100001000110001000010110000010011100101111100111101101011111111011110100000000100100010000100000101101101111100010100001110000011111010001101101000101111000111010110000101011101111010010101000111000011110010100100101101011011011100010010001100001100000001100111100101111100000001000110100100001110000001011000101100001111000011001101101010110101111100000101101100111110101111010110000001110001001010011010010100010000001110110100111100101110010001000000101000001001001111101000111101100110110110011010100001100011000001000011101000101000001111111101110010001110101011100100011000000010111001011101010010000110010111100101100101100101100100110011011010101101111110111110001110011101010101100111010111010101110100011000000010110010001010011100101110011110001111001010111010101110101011100011001111100101001001000101110001110011000100100001100010010001001000001110010000010100011101111101010001110111001011101111101011000010001010001010101011101001100101111000101010010110110000110100001000000000011010101010100110000100101011001100000000010100101010011111001010111010010000010010010111101100101011001110111001101000010000101001011001100100111000111111111111010011100100100001101111110100111011010000110101101101111101110111110100100011111010101011011001010110010100101001110111111010111101100001011111001000101000000011100000001001110111010100000000110010110111100100101010110111101000001001001110011101011101101110011011010101110111001100110001011000101111010101011010000100111010011110100001001101111110100011010000010001010100000010001100011110000101010000010111000011101100101011110111111011010100100000001001000111011100110110110010100010111111101011111101111111010011101011000000001000110110100110111100011111001001101000110000100101001110010111101110101001100100000100101100100010001101010011011111101010111011000001111101111110010001000011001101010100111111111001010001110010011";

#[allow(dead_code)]
const SIGMA_STRING: &str = "00000101100000001010010010011110001011110000011001000001001010000010100101110011100001101111011001100011101010101110011110000011111101101001000110000101101110000011010010001101101001101011000101101101010000111010010011111101100101100010100100101111101001100100111110101010100001000011110011111100100101000011111001011101110101000100111010011011001010001110011100100100001100010110011010001110001001110100010111100101100001010110011011010101100101011110100100000001111101110101001111100111110011101100001101000110011111101001010110011011000110000001010011010000101011111100001011101010010000011100010000101100111110100000111111000111011000100001001111111111101111001111100010111001111111000110111101101111111110000010111010110011011111110111101101011111101100100100011111010111000010111001011011110000110110100011010000010001100011000011011011000011011010101001110111000011100101000101011010010100111110011110110000000011010000001011001001110000011011111100101001000101111001110101111010100100010101010011110110111011010001001000001010100011110001101101101000110110010101001110100110100101100010100101010001011000100101110011111110110110000000011001100101011010000011001111111011110101010101110111000110010010101001011111011000101011000100110100110010101010101011000001001011000011111110100001100001111010100101010001000100011001100000000011000111010001110010010101011010110111010011100000110111000110010111100001100010100011110011111011110100010011100011010101001011100000100111000111000100010110010011111100010101101010011010101011001000000110101110110111110011101001100010010000000111100000011001011001011101010101011110000100100000100101011110001100000000100111011100101101010110011010001010001011110110000011100001111010000111010001100101010001010101110100000101101111011100100100111001111101001000000101100010111011110110000000110100100111011110000001010000010001011010100010100001110111101111010111000110010001100101100011110110101011100000110000001110111001000011110001101000110101001110100111011100000111000011111000111110111111111101011110"; 

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

fn find_best_candidate(candidates1: &[(u32, usize)], candidates2: &[(u32, usize)], required_len: usize) -> (u32, u32) {
    let mut best_candidate1 = candidates1[0].0;
    let mut best_candidate2 = candidates2[0].0;
    let mut min_deviation = (candidates1[0].1 as f32 - 0.25 * required_len as f32)
        + (candidates2[0].1 as f32 - 0.25 * required_len as f32);

    for &(candidate1, discrepancy1) in candidates1 {
        for &(candidate2, discrepancy2) in candidates2 {
            let deviation = (discrepancy1 as f32 - 0.25 * required_len as f32)
                + (discrepancy2 as f32 - 0.25 * required_len as f32);

            if deviation < min_deviation {
                min_deviation = deviation;
                best_candidate1 = candidate1;
                best_candidate2 = candidate2;
            }
        }
    }

    (best_candidate1, best_candidate2)
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

    let (lfsr1_candidate, lfsr2_candidate) = find_best_candidate(&lfsr1_candidates, &lfsr2_candidates, lfsr1_required_len);
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

    let (lfsr1_candidate, lfsr2_candidate) = find_best_candidate(&lfsr1_candidates, &lfsr2_candidates, lfsr1_required_len);
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