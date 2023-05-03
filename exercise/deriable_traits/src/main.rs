// 상속(derive) 매크로가 정의된 트레잇은 상속해서 사용할 수 있습니다.

// Debug
// struct나 enum의 모든 필드가 Debug로 되어있다면,
// struct와 enum에도 Debug를 상속할 수 있습니다
// Debug를 상속하면 `디버그 포맷` 과 `정돈된 디버그 포맷`을 사용할 수 있습니다

// Clone
// clone() 메서드를 호출해 값을 복제할 수 있습니다
// struct 나 enum의 모든 필드가 Clone을 상속받았으면
// 전체에 Clone을 상속할 수 있습니다

// Copy
// Clone과 밀접하게 연관되어있습니다. 특별한 marker trait 입니다
// 타입에 Copy가 구현돼 있으면, 해당 타입의 값을 넘겨줄 때 이동이 아니라 복사가 됩니다
// 그렇기 때문에 스택 영역에 맞는 작은 값에 쓰는 게 좋습니다
// 힙 영역만 사용하는 타입에는 Copy를 구현하면 안됩니다.
// Puzzle 에는 Copy를 구현할 수 없는데, String 타입은 Copy가 아니기 때문에 불가능합니다.
#[derive(Debug, Clone)]
pub struct Puzzle {
    pub num_pieces: u32,
    pub name: String,
}

// Unit 타입의 열것값 하나만 있습니다.
// Unit 타입은 Copy 입니다.
// 그래서 해당 enum은 Copy를 상속받을 수 있습니다
// 하지만 Copy만 상속받으면 컴파일 되지않습니다.
// Copy는 Clone의 하위 trait이기 떄문입니다.
// trait을 구현할 때는 그 부모 trait 을 함게 구현해야 합니다
// 소량의 데이터를 가진 작은 struct나 enum의 경우 값을 참조하고, 이동시키는 것 보다
// 복사하는게 더 빠릅니다.
#[derive(Copy, Clone)]
pub enum PuzzleType {
    Jigsaw,
}

// Trait 을 직접 구현하는 건 3단계로 구성돼 있습니다.
// 첫 번째 use 문을 사용해 trait 을 범위안으로 가지고 오기
// 두 번째 Boilerplate
// 세 번째 Implementation

// Default
impl Default for Puzzle {
    fn default() -> Self {
        Puzzle {
            num_pieces: 30,
            name: "Forest Lake".to_string(),
        }
    }
}

// PartialEq
// Eq는 마커(std::marker) trait 으로 등식이 반사적, 전이적, 대칭적으로
// 성립할 때 구현할 수 있습니다.
impl PartialEq for Puzzle {
    // self 는 self: &Self 로 풀어쓸 수 있습니다.
    // 타입은 Self 구조체 값에 대한 불변 참조 타입입니다.
    // Self를 해당 구조체 이름으로 바꿔 쓰며 Puzzle 이 되겠죠
    // PartialEq와 Eq를 모두 구현할 건지 PartialEq만 구현할 건지에 따라 다릅니다.
    fn eq(&self, other: &Self) -> bool {
        (self.num_pieces == other.num_pieces)
            && (self.name.to_lowercase() == other.name.to_lowercase())
    }
}

// Eq
// Eq 륽 구현해서 얻는 장점은 많지는 않다.
// 해시맵에서 Puzzle 을 키로 사용할 수 있다는 것 정도입니다.
impl Eq for Puzzle {}

fn main() {
    println!("Hello, world!");
    println!(
        "{:?}",
        Puzzle {
            name: "Puzzle1".to_string(),
            num_pieces: 10
        }
    ); // Debug
    println!(
        "{:#?}",
        Puzzle {
            name: "Puzzle2".to_string(),
            num_pieces: 50
        }
    ); // Pretty Debug

    let puzzle = Puzzle {
        name: "Puzzle".to_string(),
        num_pieces: 20,
    };
    // cleon() 메서드를 호출해서 Puzzle 복제
    // 러스트에는 런타임 객체 구조를 파악하고 복제할 수 있는 인터프리터가 없습니다.
    // 러스트에서 무언가 복제된다는 건, 컴파일 시점에 Clone trait 이 구현됐기 때문입니다.
    let puzzle2 = puzzle.clone();

    let puzzle3 = Puzzle {
        num_pieces: 20,
        ..Default::default()
    };
    println!("{:#?}", puzzle3);
}
