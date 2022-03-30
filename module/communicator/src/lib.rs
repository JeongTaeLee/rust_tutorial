// rust는 기본적으로 src/lib.rs만 보기 때문에 더 많은 파일을 프로젝트에 추가하고싶다면 아래와 같이 추가하자(파일 이름 == module)
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;
    #[test]
    fn it_works() {
        client::connect();
    }
}
