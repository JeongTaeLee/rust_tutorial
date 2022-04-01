// use generic_trait::*;

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl <T, U> Point<T, U> {
//     fn x(&self) -> &T {
//         &self.x
//     }

//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point { x: self.x, y: other.y }
//     }
// }


fn largests<T : PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    &largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largests(&numbers);

    println!("The largest number is {}", result);

    let numbers = vec!['y', 'm', 'a', 'q'];
    let result = largests(&numbers);

    println!("The largest number is {}", result);

    // let integer = Point{ x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };
    // let mix = Point { x: 5, y: 4 };

    // let p1 = Point { x: 5, y: 10.4 };
    // let p2 = Point { x: "Hello", y: 'c' };

    // let p3 = p1.mixup(p2);
    // println!("pt3.x = {}, pt3.y = {}", p3.x, p3.y);

    // let tweet = Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from("of course, as you probably already know, people"),
    //     reply: false,
    //     retweet: false
    // };
    
    // println!("1 new tweet: {}", tweet.summary());

    // let article = NewsArticle {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from("The Pittsburgh Penguins once again are the best
    //     hockey team in the NHL."),
    // };

    // println!("New article available! {}", article.summary());
}
