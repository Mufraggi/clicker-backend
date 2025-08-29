use async_trait::async_trait;
use fred::clients::Pool;
use fred::interfaces::HashesInterface;
use crate::domain::answer::AnswerName;
use crate::domain::poll::PollId;

#[async_trait]
pub trait PoolRedisRepositoryTrait {
     async fn increment_answer_for_poll(&self, poll_id: &PollId, answer_id:&AnswerName) ->  Result<u64, String>;
}

pub struct PoolRepository {
 pub pool: Pool
}

#[async_trait]
impl PoolRedisRepositoryTrait for PoolRepository {
    async fn increment_answer_for_poll(&self, poll_id: &PollId, answer_id: &AnswerName) -> Result<u64, String> {
        let key = format!("poll:{}:votes", poll_id);
        let answer_id = answer_id.to_string();
        match self.pool.hincrby(key, answer_id, 1).await {
            Ok(result) => Ok(result),
            Err(e)=> Err("error".to_string())
        }

    }
}