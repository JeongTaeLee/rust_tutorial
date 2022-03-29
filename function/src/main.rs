use std::i32;

fn main() {
    let x = plus_one(1);
    let y = {
        let x = 3;
        x + 1 //; 요기서 세미콜론을 찍으면 구문식이되어 반환되지 않는다. (구문식은 비어있는 튜블로 표현됨)
    }; // block

    let z = gumunsick(5);

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    // println!("The value of z is: {}", z); // 오류 발생! z는 구문식을 호출하였다
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 세미콜린이 없으면 표현식 ()
}

fn gumunsick(x: i32) {
    x + 32; //세미콜론이 찍히면 구문식 (반환값이 없는 함수)
}