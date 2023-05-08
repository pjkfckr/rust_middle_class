// 표준 라이브러리의 채널 구현체는 'std::sync::mpsc' 모듈에 있는데,
// 이건 러스트 코어 개발자들이 Firefox의 'Servo'라는 곳에 쓰려고 만든겁니다.
// 처음엔 괜찮았지만, 지금은 추천하지 않는다.
// 표준 라이브러리에 넣은 다음에야 설계가 잘못됐다는 걸 깨달았고
// 그 문제는 호환성을 깨지 않고는 고칠 수 없는 문제였죠
// 러스트 개발팀은 표준 라이브러리의 호환성을 깨지 않기로 했고,
// 이 라이브러리는 그 상태로 남게 됐습니다.
// 하지만 개발팀은 멈추지 않고 설계를 개선해 채널을 처음부터 다시 만들어
// 'crossbeam' 라이브러리에 추가했습니다.
// 'crossbeam::channel' 은 더욱 빠르고 효율적이며, 기능도 훨씬 많습니다.

// 채널은 무엇일까요?
// 채널은 단방향 큐(queue) 로, 스레드가 같은 종류의 값을 다른 스레드로 전송할 때 사용합니다.

// Send
// 모든 원시 타입은 Send 입니다.
// 'Send'는 marker trait 입니다. Copy와 Eq 처럼요
// 채널을 통해 전송할 수 있는 타입은 'Send' trait 이 구현된 타입입니다.
// 다만 Send는 직접 구현하는 게 아니라, 컴파일러가 스레드 간에 안전하게 전송할 수 있는
// 타입에 자동으로 구현해줍니다.
// 스레드 간 통신용 타입을 직접 구현해 스레드 사이에서 사용하겠다면
// 기술적으로 Send를 직접 구현할 방법이 있긴 합니다.
// 하지만 일반적인 경우에는 설계를 바꾸는게 맞습니다.
// 컴파일러가 여러분이 채널에 넣으려고 하는 값이 Send가 아니라고 한다면요

// 채널에는 두 가지 종류가 있습니다
//
// 첫 번쨰, bounded channel
// bounded channel 은 용량이 고정돼 있습니다
// channel 이 가득차면, 전송 스레드는 channel 로 다른 값을 전송하지 못하게 차단합니다
// 수신 스레드가 채널에서 값을 꺼내면 전송 스레드가 다시 재개합니다.
// bounded channel을 쓰면, 한 스레드가 다른 스레드에서 처리할 수 없을 정도로
// 많은 작업을 생성하지 못하게 할 수 있습니다.
//
// 두 번째, unbounded channel
// channel의 크기가 무한대로 커지는걸 허용합니다.
// 메모리가 부족해 동작이 중단될 떄 까지요
// 이 채널은 많은 작업을 전송할 때  유용하지만,
// 메모리가 부족해질 정도로 많아지지 않을 때만 사용해야 합니다.
//
// 위의 두 가지 채널 모두 여러 receiver를 둘 수 있습니다.
// 그중 하나만 Sender의 데이터를 받아 처리 합니다.
// 값을 받는 receiver는 따로 정해지지않습니다.
// Sender 도 여러개 가 있을 수 있습니다.
// 처음에 전송을 시도한 Sender가 channel에 먼저 아이템을 넣습니다.
// 마지막으로 여러개의 Sender와 여러개의 Receiver가 있을 수 있습니다.
// 방향은 단방향으로 고정되 있습니다.
//
// 양방향 통신을 하고 싶다면 다중 채널을 사용하면 됩니다.
// 하지만, 다중 채널을 사용할 때는설계에 매우 신경써야 합니다.
// 채널이 순환 구조로 돼있으면, deadlock(교착상태) 에 빠질 수도 있습니다.
// 예를 들어, 양방향 구조로 된 Sender와 Receiver가 있고,
// bounded channel이 가득 찰 만큼 빠르게 값을 전송하고,
// 채널에 다시 공간이 생길 때까지 채널을 차단했다고 해봅시다.
// 그럼 두 스레드 모두 다른 채널의 Receiver 측에서 차단을 해제해 수신할 수 업게 되죠
// 그래서 전송 방향은 단방향의 비순환 그래프 형태로 유지하는 게 좋습니다
// 그러면 deadlock과 같은 현상을 걱정할 필요가 없어지죠

use crossbeam::channel::{self, Receiver, Sender};
use log::info;
use std::{thread, time::Duration};

#[derive(Debug)]
enum Lunch {
    Soup,
    Salad,
    Sandwich,
    HotDog,
}

// 우리가 만든 child thread가 실행할 함수
// 함수에 이름을 넘겨서 모든 카페테리아 직원을 구분할 수 있게 할 겁니다.
// 또한 채널 Receiver에 문자열 슬라이드 참조 값을 담아 넘겨주고
// Sender에는 Lunch를 담은 값을 넘겨줍니다
fn cafeteria_worker(name: &str, orders: Receiver<&str>, lunches: Sender<Lunch>) {
    // 채널 Receiver 는 IntoIterator trait을 구현하고,
    // 채널에 뭔가가 수신될 때까지 동작을 일시중지 합니다.
    // 채널이 닫히면 for loop가 종료되고, 함수 끝에서 실행이 마무리되면
    // 동작을 실행하던 child thread도 종료됩니다.
    // 그러면 main thread 는 채널을 닫아 child thread를 완전히 종료합니다
    for order in orders {
        // 주문이 들어오면 먼저 누가 주문을 받았는지, 무엇을 주문했는지 출력합니다
        info!(target: "cafeteria_worker::order", "{} receives an order for {}", name, order);
        // match 문에 받은 주문을 그대로 넣지않고 불면 참조 값을 넣었습니다
        // 'order'를 넣게되면, order의 값을 match문 안으로 가져오려고 하기 때문입니다.
        // match 문이 끝나고 나서도 사용하고싶기 때문에,
        // match 문에서 order의 불변 참조 값을 대신 사용할 겁니다.
        let lunch = match &order {
            // match guard 도 사용했는데
            // match guard는 if문의 미니 버전 같은겁니다.
            x if x.contains("soup") => Lunch::Soup,
            x if x.contains("salad") => Lunch::Salad,
            x if x.contains("sandwich") => Lunch::Sandwich,
            _ => Lunch::HotDog,
        };

        // for loop 로 주문받은 문자열을 한 글자씩 가져와 반복문을 돌릴 때마다
        // 0.1초씩 thread 를 일시정지 합니다.
        // 식사 준비 시간은 주문한 메뉴의 길이와 비례하는 거죠
        // 많이 시켰다면 오래 걸리고, 적게 시키면 빠르게 나온다
        for _ in 0..order.len() {
            thread::sleep(Duration::from_secs_f32(0.1))
        }
        // 식사가 준비되면, 누가 어떤 음식을 돌려보낼 건지 출력합니다
        info!(target: "cafeteria_worker::Lunch", "{} sends a {:?}", name, lunch);
        // 점심 전달 채널의 Sender 쪽에서 호출합니다
        // 이를 통해 식사가 채널 너머로 전송됩니다
        // bounded channel 이면서, 채널이 가득찬 상태였다면
        // 전송이 안됐을 겁니다
        // 하지만 unbounded channel을 사용하기에 언제나 즉시 전송됩니다.
        // 이 메서드는 Result를 반환하는데, channel이 닫혀있다면 에러가 발생할 수 있기 때문이죠
        // 그래서 is_err() 메서드와 if 문을 사용해 에러가 반환되면 반복문을 중단합니다
        // 채널에 전송할 수 없기 때문이죠
        // 그러면 thread도 종료될 겁니다
        if lunches.send(lunch).is_err() {
            break;
        }
    }
}

fn main() {
    env_logger::init();
    // 주문 전송을 위해 unbounded channel을 만 듭니다.
    // channel을 만들면 Sender와 Receiver가 튜플로 반환되죠
    // 'tx'는 Sender 를 표시하고, 'rx'로 Receiver를 표시
    // unbounded() 함수를 호출할 때 터보피쉬를 사용해서 우리가 채널을 통해 전송할 타입을
    // 지정할 수도 있습니다. ex) channel::unbounded::<&str>()   with turbofish
    let (orders_tx, orders_rx) = channel::unbounded::<&str>();

    // Receiver 를 복제합니다
    // 주문받을 직원을 두명을 둘 거기 때문에 Receiver 도 두 개가 필요합니다
    // Reciver 를 복제하면 동일한 channel에 Receiver가 하나 더 생깁니다
    let orders_rx2 = orders_rx.clone();

    // 점심 전달하는 channel에도 동일한 작업을 합니다.
    let (lunches_tx, lunches_rx) = channel::unbounded();
    // 다만 이번에는 Sender측을 복제하는데, 두 직원이 각각 점심을 돌려보내야 하기 때문입니다.
    let lunches_tx2 = lunches_tx.clone();

    // 그리고 직원을 쓸 child thread 를 각각 만들고
    // 이름을 'alice' 와 'zack' 으로 지정합니다.
    // 여기서 cafeteria_worker() 함수를 호출하는 건
    // child thread 에서 동작시키기 위해서입니다.
    let alice_handle = thread::spawn(|| cafeteria_worker("alice", orders_rx2, lunches_tx2));
    let zack_handle = thread::spawn(|| cafeteria_worker("zack", orders_rx, lunches_tx));

    // 점심 주문을 전송합니다
    // 4개의 주문을 연속해서 전송해봅시다
    for order in vec![
        "polish dog",
        "caesar salad",
        "onion soup",
        "reuben sandwich",
        "bread",
    ] {
        // 주문 바은 메뉴를 출력
        info!(target: "main::ORDER", "ORDER: {}", order);
        // 채널 송신측에 있는 send() 메서드를 호출해 주문을 전달합니다.
        let _ = orders_tx.send(order);
    }
    // 추가적으로 전송할게 없다는 걸 알고 있기때문에,
    // 전송 후 즉시 orders channel 에서 Sender 떼어낼 겁니다
    // 이렇게 해도 이미 채널에 있는 것들은 영향을 받지 않지만,
    // channel이 빈 다음에는 Sender 측이 없으면 channel이 닫히게 될거고
    // 그럼 Receiver 측의 for loop 도 종료될 겁니다
    // drop 은 값을 떨어트린다는 의미입니다
    // pub fn drop<T>(_x: T){}
    // 이 함수는 버릴 값의 소유권을 가져옵니다.
    // 변수 이름 앞에 밑을 문자를 붙여 받은 값을 사용하지 말라고 경고하고 있죠
    // 타입 'T' 는 타입은 상관없다는 뜻이고요
    // 그리고 가져온 값으로 아무일도 하지않습니다.
    // 하지만, 빈 블록 {} 을 지나면 범위를 벗어났기 때문에
    // 컴파일러는 이 값을 버립니다.
    drop(orders_tx);

    // 모든 주문을 전송하고 나면, 반복문으로 완성된 점심을 돌려받은 다음
    for lunch in lunches_rx {
        // 받은 값을 출력합니다
        info!(target: "main::Lunch", "Order Up! -> {:?}", lunch);
    }

    // child thread 의 작업이 끝나길 기다렸다가 깔끔하게 종료합니다
    let _ = alice_handle.join();
    let _ = zack_handle.join();
}
