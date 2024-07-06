use std::collections::HashSet;
use tokio::process::Command;
use std::error::Error;

pub struct InstagramClient {
    username: String,
    password: String,
}

impl InstagramClient {
    pub fn new(username: String, password: String) -> Self {
        InstagramClient { username, password }
    }

    pub async fn fetch_users(&self, user_type: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let output = Command::new("python3")
            .arg("x_insta.py")
            .arg(&self.username)
            .arg(&self.password)
            .arg(user_type)
            .output()
            .await?;

        if !output.status.success() {
            return Err(format!("Failed to fetch {}: {}", user_type, String::from_utf8_lossy(&output.stderr)).into());
        }

        let users = String::from_utf8(output.stdout)?
            .lines()
            .map(|line| line.to_string())
            .collect();

        Ok(users)
    }

    pub async fn fetch_followers(&self) -> Result<Vec<String>, Box<dyn Error>> {
        self.fetch_users("followers").await
    }

    pub async fn fetch_following(&self) -> Result<Vec<String>, Box<dyn Error>> {
        self.fetch_users("following").await
    }
}
