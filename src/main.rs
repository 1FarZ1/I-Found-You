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

    let github_token: String = Input::new()
        .with_prompt("Enter your GitHub token")
        .interact_text()
        .unwrap();


    let github_client = GitHubClient::new(github_token);
    
    let following = github_client.fetch_following().await.unwrap();
    let followers = github_client.fetch_followers().await.unwrap();



    let following_set: HashSet<String> = following.iter().map(|user| user.login.clone()).collect();
    let followers_set: HashSet<String> = followers.iter().map(|user| user.login.clone()).collect();

    let not_following_back: Vec<String> = following_set
        .difference(&followers_set)
        .cloned()
        .collect();

    if not_following_back.is_empty() {
        println!("Everyone you follow follows you back!");
    } else {
        println!("Users you follow who don't follow you back:");
        for user in not_following_back {
            println!("{}", user);
        }
    }
}
