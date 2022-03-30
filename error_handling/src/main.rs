// 자세한 BACK TRACE를 보고 싶을 경우 
// RUST_BACKTRACE=1 cargo {run or build}
// --release 옵션이 없는 디버그 모드로 실행되어야함.

// Cargo.toml에 릴리즈 모드에서 아래 코드 추가 시 패닉 발생시 스택을 되감지 않고 즉시 종료 
// [profile.release]
// panic = 'abort'

use std::io::{self, Read, ErrorKind};
use std::fs::File;

fn main() {
    // panic!("crash and burn") // 패닉 발생 방법
    
    // Resultf를 이용한 예외 처리

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        // &는 참조자를 매치하고 그 값을 제공하지만, ref는 값을 매치하여 그 참조자를 제공합니다.
        // 패턴에는 ref가 필요하며 그럼으로써 error가 가드 조건문으로 소유권 이동이 되지 않고 그저 참조만 됩니다
        Err(ref err) if err.kind() == ErrorKind::NotFound => { // match .. if 는 match guard (매치가드) 라고 한다.
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                } 
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };

    // 성공이면 Ok내의 값(File)을 반환 실패하면 panic! 메소드 소출
    let f = File::open("hello.txt").unwrap();
}

// 에러 전파하기
fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e), // 함수 실행 중간에 반환하여 종료하고 싶은 경우 return 사용
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e), // 여기는 마지막이기 떄문에 return 생략 가능
    // }

    // ? 연산자
    // 성공이면 Ok 내의 값이 표현식으로부터 얻어지고
    // 실패면 Err를 반환하며 함수가 종료됨
    // 위와 같은 특성 때문에 Result를 반환하는 함수에서만 사용가능.
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?; // 위 코드랑 똑같이 작동함

    Ok(s) // 위에서 실패인 경우에만 Err를 리턴하기 때문에 Ok를 직접 리턴.
}