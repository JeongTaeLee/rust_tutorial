extern crate communicator;

use communicator::*; // 특정 아래 모든 모듈, 함수를 가져올때.
use communicator::client::connect; // 특정 모듈 또는 함수만 가져올때

fn main() {
    connect();
}