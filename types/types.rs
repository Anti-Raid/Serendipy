use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    ACTIVE,
    BANNED,
    VOTE_BANNED,
    FOLLOW_BANNED,
    PRIVATE,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartnerLinks {
    id: String,
    partner_name: String,
    partner: Partners,
    name: String,
    emoji: String,
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Partners {
    id: String,
    name: String,
    logo: String,
    category: String,
    owner: String,
    owner_image: String,
    owner_link: Option<String>,
    description: String,
    long_description: String,
    links: Vec<PartnerLinks>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Applications {
    creator_id: String,
    owner: Users,
    name: String,
    logo: String,
    token: String,
    active: bool,
    permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugins {
    id: i32,
    post_id: String,
    post: Posts,
    r#type: String,
    href: Option<String>,
    json_data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comments {
    creator_id: String,
    user: Users,
    caption: String,
    image: Option<String>,
    post: Posts,
    post_id: String,
    comment_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Upvotes {
    id: String,
    user_id: String,
    post_id: String,
    post: Posts,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Downvotes {
    id: String,
    user_id: String,
    post_id: String,
    post: Posts,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Following {
    id: String,
    user_id: String,
    user: Users,
    target_id: String,
    target: Users,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Posts {
    user_id: String,
    user: Users,
    caption: String,
    image: Option<String>,
    plugins: Vec<Plugins>,
    r#type: i32,
    post_id: String,
    upvotes: Vec<Upvotes>,
    downvotes: Vec<Downvotes>,
    comments: Vec<Comments>,
    created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    name: Option<String>,
    user_id: String,
    discord_id: Option<String>,
    usertag: String,
    bio: String,
    avatar: String,
    followers: Vec<Following>,
    following: Vec<Following>,
    badges: Vec<String>,
    state: State,
    staff_perms: Vec<String>,
    applications: Vec<Applications>,
    posts: Vec<Posts>,
    comments: Vec<Comments>,
}
