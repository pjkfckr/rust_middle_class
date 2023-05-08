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

    // 스프라이트에는 레이블이 있습니다.
    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    // 위치 지정
    player.translation = Vec2::new(31.0, 0.0);
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

    let temporary = game.add_sprite("temporary", SpritePreset::RacingCarRed);
    temporary.translation = Vec2::new(30.0, 0.0);
    temporary.layer = 1.1;

    // 2개 이상의 게임 로직을 가질 수 있습니다.
    // 로직이 추가되는 순서가 실행되는 순서가 됩니다.
    // 로직과 로직 사이 통신을 원한다면 GameState를 사용해야 합니다
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game_state.current_score += 1;
    // println!("Current score: {}", game_state.current_score);
}
