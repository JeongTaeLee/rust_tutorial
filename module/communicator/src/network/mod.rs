// mod.rs는 현재 폴더 이름을 가진 모둘의 선언을 작성하는 곳이다.

// 모듈이 공개가 아닌데 내부 함수만 공개라면 경고가 발생한다.
// fn connect() {}
pub fn connect() {}

pub mod server;