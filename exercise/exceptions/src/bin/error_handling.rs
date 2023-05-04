use std::fs::File;
use std::io;
use std::result::Result;

// 만약 Non-Recoverable Error 라면
// 즉, 프로그램이 이후의 동작을 안전하게 수행할 수 있는 방법이 없다면
// panic! 을 사용하면 됩니다.
// #[must_use]
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    // Manually panic
    panic!("crash and burn");

    // Same thing if result is a Result::Err
    // result.expect("crash and burn");

    // Same thing, but without a message
    // result.unwrap();

    // 응용 프로그램 레벨에서는 적절한 상황에 panic을 사용해도 괜찮지만,
    // 라이브러리 에서는 사용하면 안됀다.
    // 또한 Try-Catch로 감싸면 목적에 어긋납니다.
    // panic은 예외처리를 위한게 아닙니다

    // Recoverable Error 라면 '처리' 하거나 '반환' 해야합니다
    if let Err(e) = my_result {
        println!("Warning: {}", e);
    };

    // 에러가 발생했을 때 기본으로 설정되는 값을 정해놓고 싶을수도있다
    // 그땐 match 표현식을 사용하면 됩니다.
    let score = match get_saved_score() {
        Ok(x) => x,
        Err(_) => 0,
    };
    // let score = get_saved_score().unwrap_or(0);

    // 위와 같은 상황에서 저장된 점수를 가져오지 못했을 때, 어떤 에러가 발생했는지
    // 중요하지 않을 수 있습니다.
    // 기본점수를 0으로 설정하는게 중요한 상황이 있을 수 있죠.
    // 이런 상황의 대부분은 Result 의 헬퍼 메서드로 처리할 수 있습니다.
    // unwrap_or, unwrap_or_else, unwrap_or_default, unwrap_or_default
}

fn poem() -> Result<String, io::Error> {
    // Ok가 반환되면 담겨있는 값을 꺼내고, Err라면 어떻게 해야할지 모를때
    // Err로 감싸서 에러를 반환합니다.
    // 이 방식은 많이 사용되어 러스트에서 따로 문법까지 만들었습니다.
    // '물음표 연산자' 혹은 'Try 연산자' 라고 불리는 ? 를 사용하면
    // 에러가 발생했을 때, Err(e) 를 반환하는 코드를 짧게 줄일 수 있습니다.
    let f = File::open("hello.txt")?;
    // let file = match File::open("pretty_words.txt") {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
}
