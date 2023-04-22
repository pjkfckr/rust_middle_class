fn main() {
    let v = vec![6, 7, 8, 9];

    // into_iter() 는 IntoIterator 트레잇 메서드
    // IntoIterator 는 구현한 것들은 모두 for 루프에 의해 반복자로 자동 변환됩니다.
    // into_iter() 메서드는 반복자를 반환하는데,
    // 이 반복자는 구현된 컬렉션의 소유권을 가져와 컬렉션을 '소비(consume)' 합니다
    // for num in v.into_iter() {
    //     println!("{}", num);
    // }

    // 이번에는 명시적으로 into_iter()를 호출해 반복자를 가져오겠습니다.
    // 거기서 for_each() 메서드를 호출하고요
    // 이 메서드는 클로저를 인수로 받고, 클로저는 벡터의 값을 인수로 받습니다,
    // 여기서 의문은 왜 for loop를 쓰지 않고 이렇게 바꿨을까요?
    // 첫째는 보통의 경우 반복자가 for loop 보다 빠르기 때문입니다
    // 둘째는 반복자 어댑터(Iterator Adaptors)를 쓸 수 있기 때문이죠,
    // 반복자 어댑터는 함수형 프로그래밍 형식 도구로, 반복자를 이용해 통과하는 값에
    // 특정 동작을 수행할 다른 반복자를 만듭니다.
    // Iterator 트레잇 메서드 대부분이 반복자 어댑터죠
    v.into_iter().for_each(|num| println!("{}", num));

    let v = vec![6, 7, 8];
    let v2 = v
        .into_iter() // 6, 7, 8
        .map(|x: i32| x * 3) // 18, 21, 24
        .filter(|y: &i32| *y & 2 == 0) // filter 를 써서 조건에 맞지 않는 값을 삭제
        .collect::<Vec<_>>();

    // v.into_iter()  consumes v, returns owned items -> for _ in v
    // v.iter()       returns immutable references    -> for _ in &v
    // v.iter_mut()   returns mutable references      -> for _ in &mut v

    // 컬렉션의 아이템을 하나씩 가져오지 않고 컬렉션을 비우는 방법
    // drain() 메서드는 사용하는 컬렉션의 형태에 따라 다른 매개변수를 받습니다
    // 하지만, 언제나 컬렉션의 아이템 일부 또는 전체에 대한 소유권을 가진 반복자를 반환하고
    // 해당 아이템을 컬렉션에서 삭제합니다
    // 하지만 계속해서 사용할 수 있도록 컬렉션 자체는 그대로 남겨둡니다.
    // 벡터의 drain() 메서드는 벡터에서 제거할 아이템의 범위를 인수로 받습니다.
    // 최소 범위 연산자인 '..'을 넣으면 벡터를 모두 비운다는 것을 의미합니다
}
