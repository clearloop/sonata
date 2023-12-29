//! post layout.

/// The metadata of the post.
#[derive(Clone, Debug)]
pub struct Meta {
    /// The author of the post.
    pub author: String,
    /// The date of the post.
    pub date: Option<String>,
    /// The description of the post.
    pub description: String,
    /// The labels of the post.
    pub labels: Vec<String>,
    /// The title of the post.
    pub title: String,
}

/// Post layout.
#[derive(Clone, Debug)]
pub struct Post {
    /// The metadata of the post.
    pub meta: Meta,
    /// The content of the post in markdown.
    pub content: String,
}
