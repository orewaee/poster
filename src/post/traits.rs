use crate::post::entity::Post;

pub trait PostRepository {
    type Error: std::error::Error + Send + Sync + 'static;
    async fn get_by_id(&self, id: String) -> Result<Post, Self::Error>;
    async fn create(&self, new_post: Post) -> Result<String, Self::Error>;
}
