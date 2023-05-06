use anyhow::{Context, Result};
use log::info;
use puzzles::Puzzle;
use std::fs::File;

// 로그 라이브러리는 모든 기본 로거가 따르는 trait 을 통해 공통 인터페이스를 정의합니다.
// 그래서 서로 다른 라이브러리와 응용 프로그램에 있는 로그가 모두 호환되는 거죠.
// 로깅이라는 것을 다양한 라이브러리와 이를 사용하는 응용 프로그램 사이를 연결해 주는 배관이라 생각하면 됩니다.
// 배관도 그렇지만, 벽에 파이브만 붙인다고 끝이아니죠.
// 로그를 내보낼 곳이 있어야 합니다.
// 라이브러리는 로그 모듈만 사용합니다. 배관처럼요.
// 응용 프로그램에도 로그 모듈을 사용하지만, 출력을 어디론가 내보내려면
// 별도의 로깅 라이브러리가 있어야 합니다
// 로깅은 중요한 개념입니다.
// https://docs.rs/log/0.4.17/log/

// 로그 출력 라이브러리에는 다양한 종류가 있고, 목적에 따라 선택할 수 있습니다.
// 로그를 출력할 곳이 콘솔창인지, syslog인지, 일반 파일인지, Splunk 인지, 아니면 클라우드 서버인지, 응용 프로그램인지에 따라서요

// 에시로 env_logger 를 사용해보겠습니다.
// 하는 일은 환경 변수를 읽어 로그 레벨을 결정하고, 로그를 표준 에러(stderr)에 출력하는 것 뿐이죠.

// 구조화 로깅, 컨텍스트, 스팬(spans), 비동기식 코드 또는 다중 스레드를 통한
// 트레이싱(tracing) 요청 같은 고급 기능에 대해 알고싶다면, 좀 더 깊이 들어가 트레이싱 프레임워크를 공부해야 합니다.

fn get_puzzle(filename: &str) -> Result<Puzzle> {
    let fh = File::open(filename).with_context(|| format!("Failed to open {}", filename))?;
    let puzzle = Puzzle::from_file(fh).context("couldn't convert data into a puzzle")?;
    Ok(puzzle)
}

fn main() -> Result<()> {
    // env_logger::init()을 호출해 초기화
    env_logger::init();
    let puzzle = match get_puzzle("puzzle.dat").context("Couldn't get the first puzzle") {
        Ok(p) => p,
        Err(_) => Puzzle::new(),
    };
    info!("Playing puzzle: {}", puzzle.name);
    Ok(())
}
