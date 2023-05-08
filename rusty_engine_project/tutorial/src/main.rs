use rand::prelude::*;
use rusty_engine::prelude::*;

// 게임용 데이터를 저장할 위치가 필요할텐데, 엔진의 일부는 아니지만,
// 단일 프레임 이상 에 대한 엑세스가 필요합니다
// 그곳은 바로 게임 상태 구조체 입니다
// 자신의 게임 상태에 사용할 구조체를 제공할 수 있죠
struct GameState {
    high_score: u32,
    score: u32,
    gopher_index: i32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            gopher_index: 0,
            // 첫 번째 매개변수는 카운트다운 할 초의 수 이고,
            // 두 번째 매개변수는 타이머가 반복되는 여부입니다.
            spawn_timer: Timer::from_seconds(2.0, true),
        }
    }
}

fn main() {
    let mut game = Game::new();

    // BEVY 에서 바로 WindowDescriptor 구조체를 가져오는
    // window_settings를 설정하는 기능입니다.
    game.window_settings(WindowDescriptor {
        title: "Tutorial!".to_string(),
        ..Default::default()
    });

    // 음소거인 0.0 과 가장 높은 볼륨인 1.0 사이
    // 음악을 종료하려면 stop_music() 메서드 사용
    // 최소 12개의 음향 효과가 동시에 재생될 수 있습니다.
    // 정확한 수는 로컬 머신에서 사용할 수 있는 특정 하드웨어에 따라 다릅니다.
    // 음향 효과는 파이어 앤 포켓 방식으로 재생되고 사용 가능한 경우 각각 별도의 채널에서 재생되며
    // 오디오 소스의 마지막에 도달하면 종료 됩니다.
    game.audio_manager
        .play_music(MusicPreset::MysteriousMagic, 0.1);

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

    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("high_score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);

    // 2개 이상의 게임 로직을 가질 수 있습니다.
    // 로직이 추가되는 순서가 실행되는 순서가 됩니다.
    // 로직과 로직 사이 통신을 원한다면 GameState를 사용해야 합니다
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game_logic() 함수에 입력된 engine 매개변수는
    // engine 구조체에 대한 가변 참조자 입니다.
    // should_exit field
    // 이는 rusty_engine이 frame 끝에서 깔끔하게 종료되도록 true로 설정할 수 있습니다.
    // 창의 테두리에 있는 창 닫기 컨트롤을 누르거나 esc 키를 누르면 종료됩니다.
    // Q키를 눌렀을때 종료되도록 만드는 방법또한 있습니다.
    if engine.keyboard_state.just_pressed(KeyCode::Q) {
        engine.should_exit = true;
    }
    // time_since_startup_f64
    // 64bit 부동 소수점 값으로 게임 시작 이후 경과 시간을 추적합니다.
    // 이는 주기적인 애니메이션에 유용합니다. 상하 또는 좌우로 흔들리거나 원을 그리머 움직이는 애니메이션에 적용할 수 있습니다.
    // 64bit 값을 가져와서 cosine 또는 sine에 입력한 다음, 어느 정도 스케일링을 해서
    // 어떤 값에 추가해야 합니다.
    // 예를 들어, 정규 score 텍스트를 위아래로 약간 움직여 볼 수 있는데
    // 이를 실행하려면 시작한 이후의 시간을 기준으로 오프셋을 계산해야 합니다.
    // 그 다음 y 값에 추가하면 되죠
    // cos()의 출력은 -1과 1의 사이가 됩니다
    // 이를 10으로 곱하면 -10과 10 사이를 구할 수 있겠죠
    // 그 다음 engine 에서 사용할 수 있도록 이를 f32로 변환합니다
    let offset = ((engine.time_since_startup_f64 * 2.0).cos() * 3.0) as f32;

    // window_dimensions field
    // 논리적 픽셀에서 창의 너비와 높이를 설명하는 Vec2입니다.
    // 화면의 중앙이 (0, 0)이기떄문에 화면의 변은 반으로 나눈 window_dimensions 입니다
    // High Score나 Score 텍스트가 창을 늘리거나 줄여도 창 크기에 비례하여 이동하도록 수정
    let score = engine.texts.get_mut("score").unwrap();
    // window_dimensions 의 x 를 2로 나누고 약 80픽셀을 빼기
    score.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
    // window_dimensions 의 y 를 2로 나누고 약 30픽셀을 빼기
    score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;

    let high_score = engine.texts.get_mut("high_score").unwrap();
    // window_dimensions 의 -x 를 2로 나누고 약 110픽셀을 더하기
    high_score.translation.x = -engine.window_dimensions.x / 2.0 + 110.0;
    // window_dimensions 의 y 를 2로 나누고 약 30픽셀을 빼기
    high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0;

    //handle collision
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
            game_state.score += 1;
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score: {}", game_state.score);
            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;
                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High Score: {}", game_state.high_score);
            }
            // play_sfx() 메소드가 하나의 음향효과를 재생합니다.
            // 첫 번째 매개변수는 SfxPreset Enum 또는 에셋 디렉토리와 관련된 파일 경로여야 합니다
            // 두 번째 매개변수는 음악과 같이 0.0 과 1.0사이의 값에 해당하는 볼륨입니다.
            // 음악 재생을 중지하는 메소드가 있지만, 음향 효과가 시작되면 상호 작용할 방법은 없습니다.
            engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.1);
        }
    }

    // handle movement
    // 플레이어를 움직이려면 이에 대한 가변 참조자를 구해야 합니다.
    // 해당 가변 참조자는 Sprite HashMap에 있는데 unwrap 이 가능합니다.
    // 우리는 항상 그곳에 있다는 것을 알고있기 때문입니다.
    let player = engine.sprites.get_mut("player").unwrap();
    // 가로 = x, 세로 = y
    const MOVEMENT_SPEED: f32 = 100.0;
    // 방향키 뿐만 아니라 WASD로도 이동하는 로직 추가
    //
    // Up or W 입력시 위로 이동
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        // 위로 이동하려면 양수인 y가 필요
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }

    // Down or S 입력시 아래로 이동
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        // 위로 이동하려면 양수인 y가 필요
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }

    // Right or D 입력시 오른쪽으로 이동
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        // 위로 이동하려면 양수인 y가 필요
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }

    // Left or A 입력시 왼쪽으로 이동
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        // 위로 이동하려면 양수인 y가 필요
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }

    // handle mouse input
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        // 창 내부에 마우스 위치가 있는지 확인
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("gopher{}", game_state.gopher_index);
            game_state.gopher_index += 1;
            let gopher = engine.add_sprite(label.clone(), "gopher.png");
            // 왼쪽버튼을 누른 마우스의 현재위치
            gopher.translation = mouse_location;
            gopher.collision = true;
            gopher.scale = 0.3;
        }
    }

    // spawn
    // tick() 메소드는 지나간 기간을 취하는데, 이것이 바로 engine.delta의 목적입니다.
    // 타이머를 작동시키지 많으면 사실상 일시 중지된 것입니다.
    // tick()은 불변 참조자를 타이머에 리턴하므로 to_finished() 또는 just_finished() 메소드를 연결하여
    // 타이머가 완료되었거나 이 프레임 동안 타이머가 완료되었는지 확인할 수 있습니다.
    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("gopher{}", game_state.gopher_index);
        game_state.gopher_index += 1;
        let gopher = engine.add_sprite(label.clone(), "gopher.png");
        // random spawn
        // x의 범위는 (-550.0..550.0)
        // y의 범위는 (-325.0..325.0)
        gopher.translation.x = thread_rng().gen_range(-550.0..550.0);
        gopher.translation.y = thread_rng().gen_range(-325.0..325.0);
        gopher.collision = true;
        gopher.scale = 0.3;
    }

    // Reset score
    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = "Score: 0".to_string();
    }
}
