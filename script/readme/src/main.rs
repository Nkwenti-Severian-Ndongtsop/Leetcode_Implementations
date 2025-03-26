use reqwest::Client;
use serde_json::json;
use std::fs;

const LEETCODE_USERNAME: &str = "Nkwenti_Severian_Ndongtsop";

const README_PATH: &str = "README.md";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // LeetCode GraphQL API Query
    let query = json!({
        "query": "query getUserProfile($username: String!) { matchedUser(username: $username) { submitStats: submitStatsGlobal { acSubmissionNum { difficulty count } } } }",
        "variables": { "username": LEETCODE_USERNAME }
    });

    // Send GraphQL request
    let response = client
        .post("https://leetcode.com/graphql")
        .header("Content-Type", "application/json")
        .header("Referer", "https://leetcode.com")
        .json(&query)
        .send()
        .await?;

    let response_json: serde_json::Value = response.json().await?;
    
    // Extract data safely
    let stats = &response_json["data"]["matchedUser"]["submitStats"]["acSubmissionNum"];
    
    let easy = stats[1]["count"].as_i64().unwrap_or(0);
    let medium = stats[2]["count"].as_i64().unwrap_or(0);
    let hard = stats[3]["count"].as_i64().unwrap_or(0);
    let total_solved = stats[0]["count"].as_i64().unwrap_or(0);

    // Format README content
    let readme_content = format!(
        r#"# 🚀 LeetCode Solutions in Rust 🦀

![LeetCode](https://img.shields.io/badge/Solved-{}/-brightgreen)
![Language](https://img.shields.io/badge/Language-Rust-orange)

## 📖 About This Repository
This repository contains my solutions to **LeetCode** problems, implemented in **Rust**.

## 📊 LeetCode Progress
| Difficulty  | Solved |
|------------|--------|
| 🟢 Easy    | {} |
| 🔵 Medium  | {} |
| 🔴 Hard    | {} |
| **Total**  | **{}** |

## 📌 Notes & Resources
- [LeetCode Profile](https://leetcode.com/{})
- [Rust Documentation](https://doc.rust-lang.org/book/)

"#,
        total_solved, easy, medium, hard, total_solved, LEETCODE_USERNAME
    );

    // Update README
    fs::write(README_PATH, readme_content)?;
    println!("✅ README.md updated successfully!");

    Ok(())
}
