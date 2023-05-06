use log::{debug, error, info, trace, warn};

fn main() {
    // 실행 시점에, 이 다섯 가지 레벨 중 하나로 로그 레벨이 설정됩니다.
    // 설정된 레벨보다 로그 레벨이 더 높거나 같은 메세지는 출력되고,
    // 레벨이 낮은 로그는 출력되지 않습니다.
    // 예를 들어, 로그 레벨이 ERROR 로 설정되면 ERROR 레벨의 메세지만 출력됩니다
    // DEBUG 로 설정되면, DEBUG 이상 레벨의 메세지가 출력됩니다.

    // 매크로는 그 자체로 로그 레벨을 의미합니다.
    // 하지만, 첫 번째 인수로 'target'을 지정할 수도 있습니다
    // 함수에서는 이런 방식을 사용할 수 없지만, 매크로에는
    // 구문 분석에 쓸 토큰을 넘길 수 있습니다.
    error!("Serious stuff");
    warn!(target: "puzzle", "Pay attention");
    info!("Useful info");
    debug!("Extra info");
    trace!("All the things");
}
