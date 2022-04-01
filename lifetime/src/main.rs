struct ImportantExcerpt<'a> {
    part: &'a str, // 구조체 필드에 참조를 사용하고 싶다면, 라이프 타임을 명시해야한다.
}

impl<'a>  ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str { // 반환 값의 life time은 self와 같음.
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    
    let string1 = String::from("long string is long");
    
    let result;
    {
        let string2 = String::from("xyz");

        // scope 밖에서 result를 사용하는 코드가 있을 경우 에러를 발생시킨다.
        // lifetime을 통하여 string2가 result가 사용되는 스코프의 끝까지 유효할 필요가 있다고 명시했기 때문이다.
        result = longest(string1.as_str(), string2.as_str());
        
        // println!("The longest string is {}", result);
    }
    
    // 여기서 에러 발생!
    // println!("The longest string is {}", result);

    
    let ie;
    {
        let string1 = String::from("struct lifetime");
        
        // ie.part가 스코프 밖에서 사용되면 에러 발생!
        // 라이프 타임을 통해 string1 이 ie가 유효한 스코프 끝까지 유효할 필요가 있음을 명시했기 때문이다.
        ie = ImportantExcerpt {
            part: string1.as_str()
        };
    }

    // 여기서 에러 발생.
    // println!("{}", ie.part);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
