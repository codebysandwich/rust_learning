/*
* File       : main.rs
* Time       ：2024/11/14 10:10
* Author     ：sandwich
* version    ：V1.0
* Description：trait details
*/

trait Summary {
    fn summarize_author(&self) -> String;
    /// trait 默认实现
    ///
    fn summarize(&self) -> String {
        format!("(read more from: {})", self.summarize_author())
    }
}

#[allow(dead_code)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

/// 为NewsArticle实现Summary trait
///
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[allow(dead_code)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn notify(item: impl Summary) {
    println!("Breaking news!: {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    notify(tweet);
}