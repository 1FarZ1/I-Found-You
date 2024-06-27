mod api;

use clap::Parser;
use dialoguer::Input;
use api::GitHubClient;
use std::collections::HashSet;

#[derive(Parser)]
#[clap(name = "gitHelper", version = "1.0", author = "Your Name <your.email@example.com>", about = "A CLI tool to interact with GitHub")]
struct Cli {}

#[tokio::main]
async fn main() {
    let _args = Cli::parse();

    let github_token = prompt_for_github_token();

    let github_client = GitHubClient::new(github_token);

    // Fetch and compare followers and following
    match fetch_and_compare_followers(&github_client).await {
        Ok(not_following_back) => {
            print_not_following_back(&not_following_back);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn prompt_for_github_token() -> String {
    Input::new()
        .with_prompt("Enter your GitHub token")
        .interact_text()
        .unwrap_or_else(|e| {
            eprintln!("Failed to read input: {}", e);
            std::process::exit(1);
        })
}

async fn fetch_and_compare_followers(
    github_client: &GitHubClient,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let following = github_client.fetch_following().await?;
    let followers = github_client.fetch_followers().await?;

    let following_set: HashSet<String> = following.iter().map(|user| user.login.clone()).collect();
    let followers_set: HashSet<String> = followers.iter().map(|user| user.login.clone()).collect();

    // Identify users who don't follow back
    let not_following_back: Vec<String> = following_set
        .difference(&followers_set)
        .cloned()
        .collect();

    Ok(not_following_back)
}

fn print_not_following_back(not_following_back: &[String]) {
    if not_following_back.is_empty() {
        println!("Everyone you follow follows you back!");
    } else {
        println!("Users you follow who don't follow you back:");
        for user in not_following_back {
            println!("{}", user);
        }
    }
}