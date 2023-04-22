// cargo doc
// cargo doc --no-deps --open
// --no-deps 를 붙이면 나의 라이브러리에 대한 문서만 생성 합니다.
// 의존성 라이브러리에 대한 문서는 빼고 만들 수 있습니다.
// --open 을 붙이면 생성된 문서의 인덱스(index) 페이지가 기본 브라우저에 함께 열립니다.
// 이 옵션이 없으면 여러분이 직접 'target/doc/packagename/index.html' 파일을 열어야 합니다.

// 이 상수는 public 멤버인데, public 멤버는 문서화되어 웹사이트에 포함됩니다.
// 물론 private 멤버도 문서화할 수 있습니다. 하지만 웹사이트에 포함시키려면
// 'cargo doc' 명령어에 별도의 인수를 넘겨줘야 합니다

// 밑의 주석의 형태는'내부 문서화 주석' 이라고 합니다.
// Inner
/*!
Inner Docs
!*/
// 밑의 주석의 형태는 '외부 문서화 주석' 이라고 합니다.
// Outer
/**
Number of pieces in the puzzle

# History

This is a separate paragraph.
- [Clickable link](`PUZZLE_PIECES`)
- [Spawn a thread](std::thread::spawn)
- We tried `7`, but this is better
**/
pub const PUZZLE_PIECES: u32 = 42;

/// This is a Puzzle!
pub struct Puzzle {
    /// Number of pieces
    pub num_pieces: u32,
    /// Descriptive name
    pub name: String,
}
// 보통 구현 블록 자체는 문서화하지 않는데,
// 함수나 메서드를 통틀어 설명할 내용이 없기 때문입니다.
impl Puzzle {
    /// Make a new puzzle!
    pub fn new() -> Self {
        Self {
            num_pieces: PUZZLE_PIECES,
            name: "Forest Lake".into(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
