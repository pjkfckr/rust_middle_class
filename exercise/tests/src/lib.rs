// Doc-tests
// use 문 앞에 있는 '#' 앞 뒤로 공백이 존재하는데 이는 문서에 표시되지 않게 작성하는 방법이다.
/// # Example
///
/// ```
/// # use tests::snuggle;
/// let bunnies = snuggle(5);
/// assert_eq!(bunnies, 40);
/// ```
pub fn snuggle(bunnies: u128) -> u128 {
    bunnies * 8
}

// `cfg` == `config`
// #[cfg(test)] 를 사용하면
// 테스트 중일 때만 컴파일에 포함되도록 할 수 있다.
#[cfg(test)]
mod test {
    use super::*;
    use std::num::ParseIntError;

    // #[test] 특성은 Cargo 에게 이 특성이 적용된 함수는 테스트 러너에 의해
    // 실행돼야 함을 알립니다.

    // assert_eq는 PartialEq trait이 구현된 동일한 타입의 인자 두개를 받습니다. AND 연산
    // assert_ne는 not equal 입니다. XOR 연산
    // assert는 그 밖의 논리 조건을 모두 처리합니다. 그래서 연산자를 포함하여 작성해야 합니다 ex) assert!(5 >= 5)

    // 패닉이 발생하면 테스트가 실패하지만,
    // 테스트에 '#[should_panic]' 특성을 붙이면 패닉이 발생해도 통과합니다.
    #[test]
    fn snuggle_bunnies_multiply() {
        assert_eq!(snuggle(2), 16);
    }

    #[test]
    #[should_panic]
    fn scared_bunny() {
        panic!("Hop hoppity hop!");
    }

    // 테스트 작성시에 Result를 반환하려는 이유는
    // 물음표 연산자를 사용하기위해 입니다.
    #[test]
    fn bunny_result() -> Result<(), ParseIntError> {
        let num_bunnies: u64 = "four".parse()?;
        assert_eq!(num_bunnies, 4);
        Ok(())
    }

    // 테스트 실행 경로
    // Running unittests src/lib.rs (target/debug/deps/tests-453b59a16a01b37c)
    // '라이브러리' 크레이트의 테스트는 모두 같은 섹션에 있습니다.
    // 하지만 '바이너리' 크레이트의 테스트는 다른 섹션에 있습니다.
    // 바이너리는 라이브러리와 같은 crate에 있는 거 아닌가요? 라는 질문이 있을수 있다.
    // 크레이트는 두 가지 정의가 있습니다.

    // Definition
    // 1. crate = package
    // 2. crate = a library or a binary

    // 크레이트의 첫 번째 정의, 흔히 사람들끼리 얘기할 때 사용하는 크레이트는
    // `package` 를 의미합니다.
    // `crate` 를 `package`의 의미로 사용하면 패키지에는 라이브러리가 0개 또는 1개, 바이너리가 여러 개 있을 수도 있습니다.
    // 이 때는 같은 `package`에 속한 라이브러리와 바이너리를 같은 `crate`에 있다고 할 수 있습니다.

    // 크레이트의 두 번째 정의는 주로 Rust 언어와 Cargo 도구에서 사용됩니다.
    // 이때는 하나의 라이브러리 또는 바이너리를 의미하죠
    // 여기서는 모든 라이브러리 및 바이너리가 자기 자신의 `crate` 입니다.
    // 그 둘이 같은 `package` crate 에 있어도요
    // '모듈 시스템' 또는 'Cargo 의 테스트 결과 출력 방법'에 대한 글이나 공식문서 를 읽다가
    // `crate` 라는 단어를 보게 된다면, 그건 두 번째 정의의 의미로 사용된 겁니다.

    // 혼동 안하는 방법은
    // 첫 번째 `crate`의 의미를 패키지로 사용하고,
    // 두 번째 `crate`의 의미를 크레이트로 사용하는 방법이 있습니다.

    // 테스트 실행 결과에서 Doc-tests 섹션
    // Cargo는 라이브러리의 문서화 주석ㅇ서 테스트 코드를 찾아 실행합니다.
    // 오직 라이브러리에서만 찾아 실행하죠
    // 하지만, 라이브러리에서 문서화 테스트를 쓸 수 있는 것만으로도 충분히 훌륭합니다.
}
