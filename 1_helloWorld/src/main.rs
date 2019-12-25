use std::io;
use rand::Rng;

fn main(){
    println!("숫자를 입력해보고");
    println!("스트링도 입력해보고");
    let secret_number = rand::thread_rng().gen_range(1,10);
    println!("비밀번호 : {}", secret_number);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("라인읽기 실패");
    println!("입력한 스트링 값 : {}", guess);
    testFn("안녕하세요");
    println!("킹동 완성");
}

fn testFn(st){
    println!("문자열로는 어케 받누? {}", st)
}