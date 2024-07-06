mod github_client;
mod instagram_client;

use clap::Parser;
use dialoguer::{Input, Select};
use github_client::GitHubClient;
use instagram_client::InstagramClient;
use std::collections::HashSet;

#[derive(Parser)]
#[clap(name = "socialHelper", version = "1.0", author = "Your Name <your.email@example.com>", about = "A CLI tool to interact with GitHub or Instagram")]
struct Cli {}

enum Service {
    GitHub,
    Instagram,
}

#[tokio::main]
async fn main() {
    let _args = Cli::parse();

    let service = prompt_for_service();

    match service {
        Service::GitHub => {
            let github_token = prompt_for_github_token();
            let github_client = GitHubClient::new(github_token);

            match fetch_and_compare_github_followers(&github_client).await {
                Ok(not_following_back) => {
                    print_not_following_back(&not_following_back);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Service::Instagram => {
            let (username, password) = prompt_for_instagram_credentials();
            let instagram_client = InstagramClient::new(username, password);

            match fetch_and_compare_instagram_followers(&instagram_client).await {
                Ok(not_following_back) => {
                    print_not_following_back(&not_following_back);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}

fn prompt_for_service() -> Service {
    let services = vec!["GitHub", "Instagram"];
    let selection = Select::new()
        .with_prompt("Select the service you want to use")
        .default(0)
        .items(&services)
        .interact()
        .unwrap();

    match selection {
        0 => Service::GitHub,
        1 => Service::Instagram,
        _ => unreachable!(),
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

fn prompt_for_instagram_credentials() -> (String, String) {
    let username = Input::new()
        .with_prompt("Enter your Instagram username")
        .interact_text()
        .unwrap_or_else(|e| {
            eprintln!("Failed to read input: {}", e);
            std::process::exit(1);
        });

    let password = Input::new()
        .with_prompt("Enter your Instagram password")
        .interact_text()
        .unwrap_or_else(|e| {
            eprintln!("Failed to read input: {}", e);
            std::process::exit(1);
        });

    (username, password)
}

async fn fetch_and_compare_github_followers(
    github_client: &GitHubClient,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let following = github_client.fetch_following().await?;
    let followers = github_client.fetch_followers().await?;

    let following_set: HashSet<String> = following.iter().map(|user| user.login.clone()).collect();
    let followers_set: HashSet<String> = followers.iter().map(|user| user.login.clone()).collect();

    let not_following_back: Vec<String> = following_set
        .difference(&followers_set)
        .cloned()
        .collect();

    Ok(not_following_back)
}

async fn fetch_and_compare_instagram_followers(
    instagram_client: &InstagramClient,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let following = instagram_client.fetch_following().await?;
    let followers = instagram_client.fetch_followers().await?;

    let following_set: HashSet<String> = following.iter().cloned().collect();
    let followers_set: HashSet<String> = followers.iter().cloned().collect();

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
