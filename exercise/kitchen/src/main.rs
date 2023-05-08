use log::{error, info};
use std::{thread, time::Duration};

fn sleep(seconds: f32) {
    thread::sleep(Duration::from_secs_f32(seconds));
}

pub mod dad {
    use super::{info, sleep};

    pub fn cook_spaghetti() -> bool {
        info!("Cooking the spaghetti...");
        sleep(4.0);
        info!("Spaghetti is ready!");
        true
    }
}

pub mod mom {
    use super::{info, sleep};

    pub fn cook_sauce_and_let_table() {
        sleep(1.0);
        info!("Cooking the sauce...");
        sleep(2.0);
        info!("Sauce is ready! Setting the table...");
        sleep(2.0);
        info!("Table is set!");
    }
}

fn main() {
    env_logger::init();
    // child thread
    // child thread의 join()을 호출하지 않으면,
    // main thread가 종료될 때 child thread도 종료됩니다. 그러면 찌꺼기가 남죠
    // 반드시 child thread 작업이 끝나길 기다렸다가 말끔하게 종료해야 합니다.
    // 이 시점에 child thread는 OS에 의해 구성되어 background 에서 실행됩니다.
    let handle = thread::spawn(|| dad::cook_spaghetti());

    // main thread는 계속해서 mom module 에 있는 함수를 실행합니다.
    // 이제 dad module 의 함수는 child thread에서 실행되고,
    // 동시에 mom module의 함수는 main thread 에서 실행됩니다.
    mom::cook_sauce_and_let_table();

    // mom module 의 함수가 끝나면 main thread는 join() 호출 부분으로 넘어옵니다.
    // 아직 dad가 끝나지 않았으면 main thread를 잠시 멈추고 끝나길 기다립니다.
    // 그럼 bool 이 담긴 Result가 반환될 거고 unwrap_or() 함수는 child thread 에서 받은
    // true 를 반환합니다. 만약 에러가 발생하면 에러를 무시하고 false를 반환하게끔 작성했죠
    // sleep()을 쓰는 것보다 더 현실적인 예제를 만들려면 CPU를 바쁘게 만들어야 합니다.
    // fractal 계산이나 진짜 요리같은 걸 말이죠.
    if handle.join().unwrap_or(false) {
        info!("Spaghetti time! Yum!")
    } else {
        error!("Dad messed up the spaghetti. Order pizza instead?");
    }
}
