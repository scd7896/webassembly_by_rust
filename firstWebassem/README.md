# 웹 어셈블리 with rust
 심플하게 웹 어셈블리가 핫하다해서 실습을 해보는 repo이다

1. root 폴더를 만든다
2. cargo install wasm-pack 로 웹 어셈블리를 설치한다.
3. npm adduser 로 유저 정보를 등록한다(선택임) pkg에 js파일로 컴파일 되므로 그것을 사용해도 될것 같다.
4. cargo new --lib 이름 으로 rust 파일을 만든다.
5. src 에 lib.rs파일을 수정 한다.
6. cargo.toml 에 
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"

추가

7. wasm-pack build --scope npm유저이름 으로 러스트 파일을 빌드한다.
8. 러스트 프로젝트에 pkg라는 폴더가 생성되는데 여기에 js파일로 컴파일되서 제공이된다.
9. 웹 파일에서는 그 pkg파일을 임포트해서 사용할수 있다.
