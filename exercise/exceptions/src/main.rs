// 에러 처리가 에러의 전부는 아니다.
// Publish 를 목적으로 하는 라이브러리 를 위한 에러 타입을 정의하는 방법

// 첫 번째 규칙, 에러는 열거형(enum)이어야 합니다.
// Error trait을 구현하기만 하면 뭐든지 에러가 될 수 있습니다.
// 에러 구조체를 만들수도 있습니다.
// 하지만 규칙을 지키면서 작성한다면 나중에 더 편해질꺼에요
// Error 라는 단어가 포함되지 않아도 괜찮지만,
// Error 라는게 표현되지 않는다면 붙이는게 좋다.

use std::error::Error;
use std::fmt::{Display, Formatter};
// Group Errors
// 두 번쨰 규칙, 에러 열거자(enum)은 관련 있는 에러만 최소한으로 추가해야 합니다.
// ErrorKind 를 보면 파일, 소켓, 타입 관련 오류가 하나의 enum 으로 묶여있습니다.
// 모든 것이 발생 가능한 오류들이기 때문이죠
use std::io::ErrorKind;
// 하지만 정수형 값에서 발생할 수 있는 오류는 별도의 enum 으로 분리돼 있습니다.
use std::num::IntErrorKind;
// error를 enum 으로 묶을 때는 잘 생각해야한다.
// 하지만, 서로 관련 있는 것들이라면 개수가 많아져도 괜찮습니다.

// Only your errors
// 세 번째 규칙, 내가 작성한 라이브러리에서는 내가 정의한 에러를 반환해야 합니다.
// Puzzle 라이브러리에서 다른 사람이 만든 Fractal 라이브러리를 이용해
// 퍼즐용 이미지를 만든다고 가정하겠습니다.
// Fractal 라이브러리가 FractalError 를 반환하면,
// 그 에러를 Puzzle 라이브러리 사용자에게 반환 하면 안됍니다.
// 첫째는, 그렇게 하면 외부 의존성에 의해 우리가 만든 API가 중단될 수 있기때문입니다.
// 두번쨰는, 우리가 만든 API 문제를 일으키지 않고 라이브러리를 뒤에서 수정하기가 어려워지기 떄문입니다.
// 하지만 예외도 있는데, 표준 라이브러리 에러가 발생한 경우에는 그대로 전달해도 괜찮을 수 있습니다.
// 파일을 열다 발생한 IO 에러는 자체적인 에러로 변환하지 않고 그대로 사용자에게
// 전달하는 게 더 나을 수도 있습니다. 물론 개개인의 의견차이 입니다.

// 네 번째 규칙, Error enum 은 'non-exhaustive' 여야 합니다.
// #[non_exhaustive] 톡성을 붙이면 와일드카드(_) 없는 match 표현식을 쓸 수 없게 됩니다.
// 와일드카드 없이 match 문을 쓸 수 있게 두면 우리가 새로운 에러를 추가할 때 마다 client 에서도
// 추가하여 에러처리를 해야하는 상황이 발생할 수 있습니다.
// 그래서 와일드카드를 사용해서 정의되지않은 에러를 문제가 발생하지 않고 처리할 수 있습니다.

// 다섯 번째 규칙, Debug, Display, Error trait 순서대로 구현해야 합니다.
// 순서가 중요한 이유는 Error trait 가 Debug & Display 의 sub trait 이기 떄문입니다.
// pub trait Error: Debug + Display
// Debug 는 #[derive(Debug)] 추가
// Display trait 을 구현한다
// Error trait을 구현한다.

// #1: enum
// #2: Group Errors
// #3: Only YOUR Errors
// #4: Non-Exhaustive
// #5: Debug + Display + Error
// #5b: Use thiserror

pub enum FractalError {
    SadSnowFlake,
}

use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PuzzleError {
    #[error("Piece {0} doesn't fit!")]
    WontFit(u16),
    #[error("Missing a piece")]
    MissingPiece,
}

impl Display for PuzzleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PuzzleError::*;
        match self {
            MissingPiece => write!(f, "Missing a piece"),
            WontFit(n) => write!(f, "Piece {} doesn't fit!", n),
            _ => write!(f, "Something wrong!"),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
