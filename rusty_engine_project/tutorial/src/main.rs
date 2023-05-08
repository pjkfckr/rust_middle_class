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
            spawn_timer: Timer::from_seconds(1.0, false),
        }
    }
}

fn main() {
    let mut game = Game::new();

    // setup game here
    game.run(GameState::default());
}
