use benchmark::snuggle;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn snuggle_benchmark(c: &mut Criterion) {
    c.bench_function("snuggle 2", |b| b.iter(|| snuggle(black_box(2))));
}

criterion_group!(benches, snuggle_benchmark);
criterion_main!(benches);

// $ cargo bench
// bold된 글씨로 표시된 값들이 중요하다.
// 측정값 중에 outlier 가 있을 경우 이를 분류해 목록으로 표시합니다.
// 벤치마크를 수행할 컴퓨터에는 실행 중인 다른 프로그램이 적을수록 좋습니다
// 특이치 개수가 많다는 건 백그라운드 실행 중인게 너무 많다는 뜻입니다.
// 참고로, 공개 CI 서버에서는 벤치마크를 돌리지 마세요
// 측정 데이터가 모두 노출될겁니다.
// 만약 CI 서버에서 벤치마크를 돌리고 싶다면, 한번에 하나의 벤치마크만 수행하는
// 하드웨어 장치에 있는 커스텀 러너에 연결해 사용하세요

// multiply bunnies
//snuggle 2               time:   [861.48 ps 862.45 ps 863.53 ps]
//                        change: [-0.1831% +0.0348% +0.2644%] (p = 0.78 > 0.05)
//                        No change in performance detected.
// Found 11 outliers among 100 measurements (11.00%)
//  7 (7.00%) high mild
//  4 (4.00%) high severe

// add bunnies using for loop
//snuggle 2               time:   [862.32 ps 863.34 ps 864.47 ps]
//                         change: [-0.1674% +0.0549% +0.2598%] (p = 0.63 > 0.05)
//                         No change in performance detected.
// Found 7 outliers among 100 measurements (7.00%)
//   4 (4.00%) high mild
//   3 (3.00%) high severe

// shift operator
// snuggle 2               time:   [863.47 ps 864.82 ps 866.29 ps]
//                         change: [+0.1101% +0.3262% +0.5518%] (p = 0.00 < 0.05)
//                         Change within noise threshold.
// Found 2 outliers among 100 measurements (2.00%)
//   2 (2.00%) high mild

// 벤치마크 실행하는 하드웨어에 따라, 러스트 버전에 따라 결과값은 달라질 수 있다.
// 성능이 진짜 중요한 상황에 최적화가 필요하다면, 해당 환경과 비슷한 환경에서
// 성능을 측정하고, 최적화를 시도해야한다.
// html report 는 target/criterion/report/index.html 에 저장되어있고
// 그래프와 수치들을 작성해서 보여준다.
