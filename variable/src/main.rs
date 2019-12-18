fn main() {
    let mut x = 5;
    println!("x 값은 : {}", x);
    x = 6;
    println!("x 값은 : {}", x);
    let y : i8 = -99;
    println!("y 값은 : {}", y);

    let mut tup :(i32, i8, bool) = (32, 8, true);

    tup.2 = false;
    tup.0 = -20;
    tup.1 = 40;
    println!("tup의 내용, {}, {}, {}", tup.0, tup.1, tup.2);

    let arr = [5; 10];
    println!("배열 내용 {}", arr[0]);
    let gg :String = "안녕하세요".to_string();
    let test = test_fn(gg);
    println!("리턴 받은 값은 : {}" , test);
    println!("숫자 리턴 값은 : {}", number_return());
}

fn test_fn(st : String)-> String{
    println!("문자열로는 어케 받누? {}", st);
    return "난 김서버야".to_string();
}

fn number_return() -> i8{
    println!("testetset");
    return 3;
}