use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::livy::submit_to_livy;

#[derive(Serialize, Deserialize, Clone)]
pub struct QuizAnswers {
    pub q1: String,
    pub q2: String,
    pub q3: String,
}

pub const CORRECT_ANSWERS: &[(&str, &str)] = &[
    ("q1", "A"),
    ("q2", "B"),
    ("q3", "C"),
];

pub async fn score_quiz(answers: &QuizAnswers) -> Result<usize, String> {
    // Convert answers to a format for Livy
    let answers_map: HashMap<String, String> = vec![
        ("q1".to_string(), answers.q1.clone()),
        ("q2".to_string(), answers.q2.clone()),
        ("q3".to_string(), answers.q3.clone()),
    ].into_iter().collect();

    // Submit to Livy for scoring
    let score = submit_to_livy(&answers_map).await?;
    Ok(score)
}