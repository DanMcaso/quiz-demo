use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;

const LIVY_URL: &str = "http://localhost:8998";

pub async fn submit_to_livy(answers: &HashMap<String, String>) -> Result<usize, String> {
    let client = Client::new();
    
    // Create a Livy session
    let session_response = client
        .post(format!("{}/sessions", LIVY_URL))
        .json(&json!({ "kind": "spark" }))
        .send()
        .await
        .map_err(|e| format!("Failed to create Livy session: {}", e))?;
    
    let session_json: serde_json::Value = session_response.json().await.map_err(|e| format!("Failed to parse session response: {}", e))?;
    let session_id = session_json["id"].as_u64().ok_or("No session ID")?;
    
    // Create Spark code for scoring
    let spark_code = format!(
        r#"
        val answers = Seq({})
        val correctAnswers = Seq({})
        val df = spark.createDataFrame(answers).toDF("question", "answer")
        val correctDf = spark.createDataFrame(correctAnswers).toDF("question", "correct_answer")
        val joinedDf = df.join(correctDf, "question")
        val score = joinedDf.filter(col("answer") === col("correct_answer")).count()
        println(score)
        "#,
        answers.iter()
            .map(|(k, v)| format!("(\"{}\", \"{}\")", k, v))
            .collect::<Vec<_>>()
            .join(", "),
        crate::quiz::CORRECT_ANSWERS
            .iter()
            .map(|(k, v)| format!("(\"{}\", \"{}\")", k, v))
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Submit code to the session
    let statement_response = client
        .post(format!("{}/sessions/{}/statements", LIVY_URL, session_id))
        .json(&json!({ "code": spark_code }))
        .send()
        .await
        .map_err(|e| format!("Failed to submit statement to Livy: {}", e))?;
    
    let statement_json: serde_json::Value = statement_response.json().await.map_err(|e| format!("Failed to parse statement response: {}", e))?;
    let statement_id = statement_json["id"].as_u64().ok_or("No statement ID")?;

    // Poll for result
    loop {
        let status_response = client
            .get(format!("{}/sessions/{}/statements/{}", LIVY_URL, session_id, statement_id))
            .send()
            .await
            .map_err(|e| format!("Failed to get statement status: {}", e))?;
        
        let status_json: serde_json::Value = status_response.json().await.map_err(|e| format!("Failed to parse status response: {}", e))?;
        let state = status_json["state"].as_str().ok_or("No state")?;
        
        if state == "available" {
            let output = status_json["output"].get("data").and_then(|d| d.get("text/plain")).and_then(|t| t.as_str());
            if let Some(score_str) = output {
                let score = score_str.trim().parse::<usize>().map_err(|e| format!("Failed to parse score: {}", e))?;
                // Clean up session
                client.delete(format!("{}/sessions/{}", LIVY_URL, session_id)).send().await.ok();
                return Ok(score);
            } else {
                return Err("No output data".to_string());
            }
        } else if state == "error" {
            return Err("Execution error".to_string());
        }
        
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}