use redis::{AsyncCommands, Client};
use uuid::Uuid;

pub struct RedisStore {
    client: Client,
}

impl RedisStore {
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::open(url).unwrap(),
        }
    }

    pub async fn get_elo(&self, player: Uuid) -> i32 {
        let mut conn = self.client.get_async_connection().await.unwrap();
        conn.get(format!("player:{}:elo", player))
            .await
            .unwrap_or(1200)
    }

    pub async fn set_elo(&self, player: Uuid, elo: i32) {
        let mut conn = self.client.get_async_connection().await.unwrap();
        let _: () = conn
            .set(format!("player:{}:elo", player), elo)
            .await
            .unwrap();
    }
}
