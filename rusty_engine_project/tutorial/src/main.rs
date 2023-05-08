use rusty_engine::prelude::*;

// 게임용 데이터를 저장할 위치가 필요할텐데, 엔진의 일부는 아니지만,
// 단일 프레임 이상 에 대한 엑세스가 필요합니다
// 그곳은 바로 게임 상태 구조체 입니다
// 자신의 게임 상태에 사용할 구조체를 제공할 수 있죠
struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0, false), // 초당 60프레임
        }
    }
}

fn main() {
    let mut game = Game::new();

    // 스프라이트에는 label이 있습니다.
    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    // 위치 지정
    player.translation = Vec2::new(0.0, 0.0);
    // 회전
    // 파이를 향해 가면서 위쪽으로 회전한 다음 파이에서는 왼쪽을 향하게 됩니다.
    // 계속 진행하면서 아래쪽을 향하기 시작한 다음 회전하여 2파이에 도달하면 시작했던 위치로 오게 됩니다.
    // 파이/2 는 위를 향해야 합니다
    // player.rotation = std::f32::consts::FRAC_PI_2;
    player.rotation = SOUTH_WEST;
    // scale 기본값은 1.0, 즉 100% 입니다.
    // 더 작은 값은 sprite를 축소하고, 더 큰 값은 sprite를 확대합니다
    player.scale = 1.0;
    // Sprite 는 모두 0.0 레이어로 기본 설정됩니다
    // Sprite 의 레이어는 Sprite가 렌더링되는 순서를 결정합니다
    // 즉, Sprite가 다른 Sprite위에 렌더링 되는 순서를 말합니다.
    player.layer = 1.0;
    // 충돌을 활성화
    // 각 Sprite 별로 충돌을 원한다면 모두가 활성화 되어있어야 하며,
    // 만약 일부 Sprite가 활성화 되지않았다면, 해당 Sprite들은 충돌이 일어나지 않습니다.
    player.collision = true;

    // let temporary = game.add_sprite("temporary", SpritePreset::RacingCarRed);
    // temporary.translation = Vec2::new(30.0, 0.0);
    // temporary.layer = 1.1;

    let car1 = game.add_sprite("car1", "gopher.png");
    car1.translation = Vec2::new(300.0, 0.0);
    car1.collision = true;
    car1.scale = 0.3;

    // 2개 이상의 게임 로직을 가질 수 있습니다.
    // 로직이 추가되는 순서가 실행되는 순서가 됩니다.
    // 로직과 로직 사이 통신을 원한다면 GameState를 사용해야 합니다
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Collider 는 2개의 Sprite 간 충돌이 발생했는지 감지하는 데 사용되는 볼록 다각형입니다.
    // 화면에서 흰색 선이 있는 다각형으로 렌더링 됩니다
    // Collider는 Sprite가 사용하는 이미지 파일과 동일한 파일명 및 경로를 가진 파일에 저장됩니다.
    // 확장자는 collider 입니다.
    // 유효한 collider 파일이 존재한다면 자동으로 로딩됩니다.
    // 유효한 collider 파일이 존재하지 않다면,
    // Collider 라고 하는 프로그램 예제를 사용하여 collider 파일을 만들 수 있습니다.
    // engine.show_colliders = true;

    // 이벤트 처리
    // collision_events 벡터의 모든 이벤트에 반복문을 적용할 수 있습니다.

    for event in engine.collision_events.drain(..) {
        // state는 CollisionState enum 의 Begin 또는 End에 해당합니다
        // pair 는 문자열 튜플인 CollisionPair 입니다
        // 2개의 묹열은 충돌과 관련된 2개의 Sprite label을 나타냅니다
        // label의 순서는 비결정론적이라서 어떤 Sprite label이 첫 번째이고 두번째인지
        // 미리 알 수 있는 방법은 없습니다.
        //CollisionEvent {
        //     state: Begin,
        //     pair: CollisionPair(
        //         "car1",
        //         "player",
        //     ),
        // }
        // CollisionEvent {
        //     state: End,
        //     pair: CollisionPair(
        //         "car1",
        //         "player",
        //     ),
        // }
        // 충돌 했을 경우, 점수를 얻고 다른 Sprite를 제거해야 합니다
        // event.pair.one_starts_with() 은
        // Label 중 하나가 지정한 것으로 정확히 시작하는 경우 true를 리턴합니다
        // 즉 event.state가 CollisionState::Begin 이고
        // pair에서 player label이 있을 경우 입니다
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            // 튜플의 어떤항목이 플레이어 label인지 다른 레이블인지 알지 못하므로 pair 두개를 넣고
            for label in [event.pair.0, event.pair.1] {
                // label이 player가 아닌 경우 엔진의 Sprite HashMap에서 해당 label을 제거
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
            // 점수 += 1
            game_state.current_score += 1;
            println!("Current Score: {}", game_state.current_score);
        }
    }

    // 플레이어를 움직이려면 이에 대한 가변 참조자를 구해야 합니다.
    // 해당 가변 참조자는 Sprite HashMap에 있는데 unwrap 이 가능합니다.
    // 우리는 항상 그곳에 있다는 것을 알고있기 때문입니다.
    let player = engine.sprites.get_mut("player").unwrap();
    // 상수에 delta_f32 값을 곱해서 부드러운 움직임을 구현
    player.translation.x += 100.0 * engine.delta_f32;
}
