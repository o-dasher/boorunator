#[cfg(test)]
mod danbooru {
    use rusty_booru::{
        danbooru::{client::DanbooruClient, DanbooruRating},
        shared::{
            client::{QueryDispatcher, WithClientBuilder},
            Sort,
        }, generic::client::GenericClient,
    };

    #[tokio::test]
    async fn get_posts_with_tag() {
        let posts = DanbooruClient::builder()
            .default_url("https://danbooru.donmai.us")
            .query(|q| q.tag("my_chemical_romance"))
            .get()
            .await;

        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_rating() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .query(|q| q.tag("kafuu_chino").rating(DanbooruRating::General))
            .get()
            .await;

        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_sort() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .query(|q| q.tag("kafuu_chino").sort(Sort::Rating))
            .get()
            .await;

        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_blacklist_tag() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .query(|q| q.tag("kafuu_chino").blacklist_tag(DanbooruRating::Explicit))
            .get()
            .await;

        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_posts_with_limit() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .query(|q| q.tag("kafuu_chino").limit(3))
            .get()
            .await;

        assert!(posts.unwrap().len() == 3);
    }

    #[tokio::test]
    async fn get_posts_multiple_tags() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .query(|q| q.tag("kafuu_chino").tag("bangs").limit(3))
            .get()
            .await;

        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_random_posts() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .query(|q| q.tag("kafuu_chino").random())
            .get()
            .await;

        assert!(!posts.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_post_by_id() {
        let post = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .dispatch()
            .get_by_id(9423)
            .await;

        assert_eq!(
            "15a1b49c26f5c684807a2f0b838f9e4c",
            post.unwrap().unwrap().md5.unwrap()
        );
    }

    #[tokio::test]
    async fn query_too_many_tags() {
        let posts = DanbooruClient::builder()
            .default_url("https://testbooru.donmai.us")
            .query(|q| q.tag("kafuu_chino").tag("loli").tag("maid").random())
            .get()
            .await;

        assert!(posts.is_err());
    }

    #[test]
    fn parse_rating_tags() {
        assert_eq!("explicit", DanbooruRating::Explicit.to_string());
        assert_eq!("questionable", DanbooruRating::Questionable.to_string());
        assert_eq!("sensitive", DanbooruRating::Sensitive.to_string());
        assert_eq!("general", DanbooruRating::General.to_string());
    }

    #[test]
    fn parse_sort_tags() {
        assert_eq!("id", Sort::Id.to_string());
        assert_eq!("score", Sort::Score.to_string());
        assert_eq!("rating", Sort::Rating.to_string());
        assert_eq!("user", Sort::User.to_string());
        assert_eq!("height", Sort::Height.to_string());
        assert_eq!("width", Sort::Width.to_string());
        assert_eq!("source", Sort::Source.to_string());
        assert_eq!("updated", Sort::Updated.to_string());
    }
}
