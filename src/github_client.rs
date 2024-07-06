use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
}

pub struct GitHubClient {
    client: reqwest::Client,
    token: String,
}

impl GitHubClient {
    pub fn new(token: String) -> Self {
        GitHubClient {
            client: reqwest::Client::new(),
            token,
        }
    }

    async fn fetch_users(&self, endpoint: &str) -> Result<Vec<User>, Error> {
        let url: String = format!("https://api.github.com/user/{}", endpoint);
        let response = self
            .client
            .get(&url)
            .header("User-Agent", "gitHelper")
            .header("Authorization", format!("token {}", self.token))
            .send()
            .await?;
        let users: Vec<User> = response.json().await?;
        Ok(users)
    }

    pub async fn fetch_following(&self) -> Result<Vec<User>, Error> {
        self.fetch_users("following").await 
    }

    pub async fn fetch_followers(&self) -> Result<Vec<User>, Error> {
        self.fetch_users("followers").await
    }
}
