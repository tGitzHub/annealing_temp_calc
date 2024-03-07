use std::io;

fn main() {
    let primer_p4 = "acgagtactggcagcgggagaaccgcgcctatgcgc";
    let reference_p4 = "ACGAGTACTGGCAGCGGATGAACCGCGCCTATGCGC";
    let tm_1 = calc_from_strings(primer_p4, reference_p4);
    println!("Annealing temperature (Tm) for primer p4: {:.2} °C", tm_1);

    let primer_0 = "gagtactggcagcgggagaaccgcgcctatgc";
    let reference_0 = "GAGTACTGGCAGCGGATGAACCGCGCCTATGC";
    let tm_2 = calc_from_strings(primer_0, reference_0);
    println!("Annealing temperature (Tm) for primer 0: {:.2} °C", tm_2);

    let second_binding_site = "ATGTGCGAGGACTATCGCGCCGGCGCCTATGC";
    let second_binding_site_p4 = "TGATGTGCGAGGACTATCGCGCCGGCGCCTATGCGG";

    let tm_3 = calc_from_strings(primer_0, second_binding_site);
    let tm_4 = calc_from_strings(primer_p4, second_binding_site_p4);
    println!("Annealing temperature (Tm) for primer 0 and second binding site: {:.2} °C", tm_3);
    println!("Annealing temperature (Tm) for primer p4 and second binding site: {:.2} °C", tm_4);

    let primer_m145l = "gtactggcagcggctgaaccgcgccta";
    let reference_m145l = "GTACTGGCAGCGGATGAACCGCGCCTA";
    
    let tm_m145l = calc_from_strings(primer_m145l, reference_m145l);
    println!("Annealing temperature (Tm) for primer M145L: {:.2} °C", tm_m145l);
    
    let primer_m145i = "cgagtactggcagcggattaaccgcgcct";
    let reference_m145i = "CGAGTACTGGCAGCGGATGAACCGCGCCT";

    let tm_m145i = calc_from_strings(primer_m145i, reference_m145i);
    println!("Annealing temperature (Tm) for primer M145I: {:.2} °C", tm_m145i);
    /*
    let mismatch_percentage = calculate_mismatch_percentage(primer_p4, second_binding_site_p4);
    let mismatch_percentage_2 = calculate_mismatch_percentage(primer_0, second_binding_site);
    println!("Mismatch percentage between primer p4 and second binding site: {:.2}%", mismatch_percentage);
    println!("Mismatch percentage between primer 0 and second binding site: {:.2}%", mismatch_percentage_2);
     */
}

fn calculate_gc_content(chain: &str) -> f64 {
    let chain_lower = chain.to_lowercase();
    // Count the occurrences of 'G' and 'C' in the chain
    let gc_count = chain_lower.chars().filter(|&c| c == 'g' || c == 'c').count() as f64;

    // Calculate the percentage of 'G' and 'C' in the chain
    (gc_count / chain.len() as f64) * 100.0
}

fn calculate_mismatch_percentage(chain1: &str, chain2: &str) -> f64 {
    let chain1_lower = chain1.to_lowercase();
    let chain2_lower = chain2.to_lowercase();

    // Count the number of mismatched positions
    let mismatch_count = chain1_lower.chars().zip(chain2_lower.chars()).filter(|&(c1, c2)| c1 != c2).count() as f64;

    // Calculate the percentage of mismatch
    (mismatch_count / chain1.len() as f64) * 100.0 
}

fn calc_from_strings(chain1: &str, chain2: &str) -> f64 {
    let gc_primer = calculate_gc_content(chain1);
    let length = chain1.len() as f64;
    let mismatch_percentage = calculate_mismatch_percentage(chain1, chain2);
    81.5 + 0.41 * (gc_primer) - (675.0 / (length)) - mismatch_percentage
    //println!("Annealing temperature (Tm): {:.2} °C", tm);
    //println!("Used strings: {} and {}", chain1, chain2);
}

#[allow(dead_code)]
fn calc_from_user_input() {
    // Get input strings from the user
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the primer nucleotide chain: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input.");

    println!("Enter the reference nucleotide chain: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input.");

    // Remove leading and trailing whitespaces
    let chain_primer = input1.trim();
    let chain_reference = input2.trim();

    // Calculate %GC for each chain
    let gc_primer = calculate_gc_content(chain_primer);
    println!("GC content of primer: {:.2}%", gc_primer);
    //let gc_reference = calculate_gc_content(chain_reference);

    // Calculate the length of the strings
    let length = chain_primer.len() as f64;
    println!("Length of primer: {}", length);

    // Calculate %mismatch between the two strings
    let mismatch_percentage = calculate_mismatch_percentage(chain_primer, chain_reference);
    println!("Mismatch percentage: {:.2}%", mismatch_percentage);

    // Calculate the annealing temperature using the formula
    let tm = 81.5 + 0.41 * (gc_primer) - (675.0 / (length)) - mismatch_percentage;

    println!("Annealing temperature (Tm): {:.2} °C", tm);
}