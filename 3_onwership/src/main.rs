fn main() {
    // let s1 = String::from("hello world");
    // let s2 = s1; 
    // s2 에 s1이 대입이 되면서 s1이라는 변수명을 해제시키고 그 주소를 s2가 들고간다
    let s1 = String::from("test clone");
    let s2 = s1.clone();
    println!("{}", s1);

    let giveString = give_string();
    let give_and_take = take_and_give_sring(giveString.clone());

    println!("give : {}, taken : {}", giveString, give_and_take);
}

fn give_string() -> String {
    let some_string = String::from("giveString");
    return some_string
}

fn take_and_give_sring(resource : String) -> String{
    return resource
}