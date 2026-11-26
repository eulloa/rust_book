fn main() {
    let post = SocialPost {
        username: String::from("eulloa"),
        content: String::from("fun with Rust traits"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summary());

    let second_post = SocialPost {
        username: String::from("eulloa345"),
        content: String::from("fun with Rust traits 345"),
        reply: false,
        repost: false,
    };

    println!("another new post: {}", second_post.default_summary());
}

pub trait Summary {
    fn summary(&self) -> String;
}

pub trait SummaryWithDefaultImplementation {
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

// structs
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool
}

impl Summary for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for SocialPost {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl SummaryWithDefaultImplementation for SocialPost {}

