Quiz Demo with Livy and Verifiable Off-Chain Compute
This is a simple Rust-based web application that demonstrates a quiz system using Livy to interact with Apache Spark for scoring quiz answers. It also includes a simulated verifiable off-chain compute feature using a hash-based zero-knowledge proof (ZKP) to verify the score's integrity without revealing the answers.
Features

Submit quiz answers via a POST request.
Score the quiz using Apache Spark via Livy.
Generate a simplified ZKP for the score.
Verify the ZKP to ensure the score is correct.

Prerequisites

Apache Spark: Installed with SPARK_HOME environment variable set.
Livy Server: Installed and running at http://localhost:8998. Configure Livy to connect to your Spark cluster.
Start Livy: livy-server start


Rust: Installed via rustup.
A tool like curl or Postman for testing API endpoints.

Installation

Extract the Zip File:
unzip quiz_demo.zip
cd quiz_demo


Build the Project:
cargo build

This will download dependencies and compile the application.


Usage

Run the Server:
cargo run

The server will start listening on http://127.0.0.1:8080.

Submit a Quiz:Use curl to submit answers to the quiz (correct answers: q1: "A", q2: "B", q3: "C"):
curl -X POST http://127.0.0.1:8080/submit_quiz \
     -H "Content-Type: application/json" \
     -d '{"answers": {"q1": "A", "q2": "B", "q3": "C"}}'

Example Response:
{
    "score": 3,
    "total": 3,
    "proof": "some_hex_hash",
    "answers": {"q1": "A", "q2": "B", "q3": "C"}
}


Verify the Proof:Use the response from the submit endpoint (including score, answers, and proof):
curl -X POST http://127.0.0.1:8080/verify_proof \
     -H "Content-Type: application/json" \
     -d '{"score": 3, "total": 3, "proof": "some_hex_hash", "answers": {"q1": "A", "q2": "B", "q3": "C"}}'

Example Response:
{
    "verified": true,
    "message": "Score is valid"
}



Notes

Livy Integration: The application creates a Livy session, submits Spark code for scoring, polls for the result, and cleans up the session. Ensure Livy is properly configured for your Spark setup. In production, handle errors and timeouts more robustly.
ZKP Simulation: The ZKP is a simple SHA-256 hash of the score and answers. For real verifiable compute, integrate a library like risc0-zkvm to generate actual zero-knowledge proofs.
Customization: Modify src/quiz.rs to add more questions or change correct answers.
Troubleshooting:
If Livy fails to connect, check the server is running and accessible.
Ensure Spark is in the PATH and properly configured.


Limitations: This is a demo; the ZKP is not cryptographically secure for production use. Livy submissions assume a local setup.

For questions or improvements, feel free to modify the code!