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

    // setup game here

    // 2개 이상의 게임 로직을 가질 수 있습니다.
    // 로직이 추가되는 순서가 실행되는 순서가 됩니다.
    // 로직과 로직 사이 통신을 원한다면 GameState를 사용해야 합니다
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    game_state.current_score += 1;
    println!("Current score: {}", game_state.current_score);
}
