use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use quiz::{QuizAnswers, score_quiz};
use zkp::{generate_zkp, verify_zkp};

#[derive(Deserialize)]
struct QuizSubmission {
    answers: QuizAnswers,
}

#[derive(Serialize, Deserialize, Clone)]
struct QuizResponse {
    score: usize,
    total: usize,
    proof: String,
    answers: QuizAnswers,
}

#[derive(Serialize)]
struct VerificationResponse {
    verified: bool,
    message: String,
}

async fn submit_quiz(data: web::Json<QuizSubmission>) -> impl Responder {
    match score_quiz(&data.answers).await {
        Ok(score) => {
            let total = 3; // Number of questions
            let proof = generate_zkp(score, &data.answers);
            HttpResponse::Ok().json(QuizResponse { score, total, proof, answers: data.answers.clone() })
        }
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn verify_proof(data: web::Json<QuizResponse>) -> impl Responder {
    let verified = verify_zkp(data.score, &data.answers, &data.proof);
    let message = if verified { "Score is valid" } else { "Invalid proof" };
    HttpResponse::Ok().json(VerificationResponse { verified, message: message.to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/submit_quiz", web::post().to(submit_quiz))
            .route("/verify_proof", web::post().to(verify_proof))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}