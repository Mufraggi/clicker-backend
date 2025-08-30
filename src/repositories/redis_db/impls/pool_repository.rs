use crate::domain::answer::AnswerName;
use crate::domain::poll::PollId;
use async_trait::async_trait;
use fred::clients::Pool;
use fred::interfaces::HashesInterface;

#[async_trait]
pub trait PoolRedisRepositoryTrait {
    async fn increment_answer_for_poll(
        &self,
        poll_id: &PollId,
        answer_id: &AnswerName,
    ) -> Result<u64, String>;
    async fn get_poll_results(&self, poll_id: &PollId) -> Result<std::collections::HashMap<AnswerName, u64>, String>;
}

pub struct PoolRepository {
    pub pool: Pool,
}
impl PoolRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
    fn get_key(&self, poll_id: &PollId) -> String {
        format!("poll:{}:votes", poll_id)
    }
}

#[async_trait]
impl PoolRedisRepositoryTrait for PoolRepository {
     async  fn increment_answer_for_poll(
        &self,
        poll_id: &PollId,
        answer_id: &AnswerName,
    ) -> Result<u64, String> {
        let key = self.get_key(poll_id);
        let answer_id = answer_id.to_string();
        match self.pool.hincrby(key, answer_id, 1).await {
            Ok(result) => Ok(result),
            Err(e) => Err("error".to_string()),
        }
    }
    async fn get_poll_results(&self, poll_id: &PollId) -> Result<std::collections::HashMap<AnswerName, u64>, String> {
        let key = self.get_key(poll_id);
        match self.pool.hgetall::<std::collections::HashMap<String, u64>, _>(key).await {
            Ok(redis_result) => {
                let mut result = std::collections::HashMap::new();
                for (key, value) in redis_result {
                    let answer_name = AnswerName::new(key);
                    result.insert(answer_name, value);
                }
                Ok(result)
            },
            Err(e) => Err(format!("Erreur lors de la récupération des résultats du poll: {}", e)),
        }
    }
}
