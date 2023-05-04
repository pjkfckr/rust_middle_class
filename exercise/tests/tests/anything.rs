use hello::snuggle;

// 통합 테스트에서는 여러 개의 함수와 구조체 등등의 것을 한번에 테스트 진행
#[test]
fn it_works_from_outside() {
    assert_eq!(snuggle(4), 32)
}
