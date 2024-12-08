fn main() {
    
struct Candidate {
    name: String,
    years_of_experience: usize,
}


fn main() {
    
    let candidates = vec![
        Candidate {
            name: "Alice".to_string(),
            years_of_experience: 5,
        },
        Candidate {
            name: "Bob".to_string(),
            years_of_experience: 10,
        },
        Candidate {
            name: "Charlie".to_string(),
            years_of_experience: 8,
        },
        Candidate {
            name: "David".to_string(),
            years_of_experience: 12,
        },
    ];

    
    let top_candidate = find_top_candidate(&candidates);

    match top_candidate {
        Some(candidate) => println!(
            "The candidate with the highest experience is {} with {} years of experience.",
            candidate.name, candidate.years_of_experience
        ),
        None => println!("No candidates found."),
    }
}

fn find_top_candidate(candidates: &[Candidate]) -> Option<&Candidate> {
    candidates.iter().max_by_key(|c| c.years_of_experience)
}

}
