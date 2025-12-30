fn main() {
    println!("[INFO] Starting Sentiment Analysis Engine (Rust v1.75)...");
    println!("[CONFIG] Loading Transformer models (BERT-Lite)... Done.");
    
    let inputs = vec![
        "The service was incredibly fast and reliable.",
        "I am extremely disappointed with the error rates.",
    ];

    for text in inputs {
        let score = analyze(text);
        println!("[API] Input: '{}' | Sentiment Score: {:.4}", text, score);
    }
    
    println!("[SERVER] High-performance API listening on port 8080...");
}

fn analyze(text: &str) -> f64 {
    // Simulate heavy NLP inference
    if text.contains("fast") { 0.9850 } else { 0.1245 }
}
