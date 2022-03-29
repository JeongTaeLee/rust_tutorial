fn main() {
    // if_flow
    loop_flow();
}

fn if_flow() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
        // "six" // 이 조건 표현식에서는 반환하는 값의 타입은 모두 같아야한다
    }; 

    println!("The value of number is: {}", number);
}

fn loop_flow() {
    let a = [10, 20, 30, 40, 50];
    
    // 아래 구문은 오류가 발생하기 쉽고 느리다.
    // let mut index = 0;
    // while index < 5 {
    //    println!("the value is: {}", a[index]); // 컴파일러가 이 부분에서 요소 범위에 대한 조건 검사를 수행하는 런타임 코드를 추가하기 때문.
    //    index = index + 1;
    // }

    // 다음은 안정성 있고 효율적인 코드다
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    println!("");

    // 특정 범위를 반복하는 for
    for number in 1..4 {
        println!("{}!", number)
    }

    println!("");
    
    // 특정 범위를 역순으로 반복하는 for
    for number in (1..4).rev() {
        println!("{}!", number)
    } 
}
