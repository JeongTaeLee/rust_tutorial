pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

// 에러! 외부 타입에 대한 외부 트레잇 구현은 허용하지 않는다. (고아 규칙)
// impl<T> Display for Vec<T> {
//     fn summary(&self) -> String {
//         todo!()
//     }
// }

// 다음은 가능 : Summrizable이 현재 크레이트의 트레잇이기 때문
// impl<T> Summarizable for Vec<T> {
// }

//use std::fmt::Display;
// 다음은 가능 : NewsArticle이 현재 크레이트의 구조체이기 때문
// impl Display for NewsArticle {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

use std::fmt::{Debug, Display};

pub fn notify<T: Summarizable>(item: &T) {
    println!("Breaking news! {}", item.summary());
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn author_summary(&self) -> String {
        format!("@{}", self.headline)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }

    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {

// }

// where 절로 깔끔하게 나타낼 수 있음
fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    return 0;
}
