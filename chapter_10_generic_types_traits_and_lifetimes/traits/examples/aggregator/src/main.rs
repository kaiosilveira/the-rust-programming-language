pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more from {}...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Email {
    pub from: String,
    pub to: String,
    pub subject: String,
}

impl Summary for Email {}

fn main() {
    let tweet = Tweet {
        username: String::from("silveirakaio"),
        content: String::from("Of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Click bait!"),
        location: String::from("Lisbon, Portugal"),
        author: String::from("Kaio Silveira"),
        content: String::from("Nothing useful here, though. Thanks for the clickbait btw"),
    };

    let email = Email {
        from: String::from("gab@test.com"),
        to: String::from("kaio@test.com"),
        subject: String::from("Hey hey"),
    };

    println!("{}", tweet.summarize());
    println!("{}", article.summarize());
    println!("{}", email.summarize());
}
