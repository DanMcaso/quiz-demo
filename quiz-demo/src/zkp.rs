use sha2::{Digest, Sha256};
use crate::quiz::QuizAnswers;

pub fn generate_zkp(score: usize, answers: &QuizAnswers) -> String {
    // Simplified ZKP: Hash the score and answers
    let mut hasher = Sha256::new();
    let input = format!("{}:{}:{}:{}", score, answers.q1, answers.q2, answers.q3);
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn verify_zkp(score: usize, answers: &QuizAnswers, proof: &str) -> bool {
    let expected_proof = generate_zkp(score, answers);
    expected_proof == proof
}