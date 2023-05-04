use criterion::{criterion_group, criterion_main, Criterion};
use hello::sploosh;

pub fn sploosh_benchmark(c: &mut Criterion) {
    c.bench_function("sploosh ", |b| b.iter(|| sploosh(8, 9, 10)));
}

criterion_group!(benches, sploosh_benchmark);
criterion_main!(benches);

// sploosh Before Refactor
// Gnuplot not found, using plotters backend
// sploosh                 time:   [1.1493 ns 1.1504 ns 1.1515 ns]
// Found 6 outliers among 100 measurements (6.00%)
//   6 (6.00%) high mild

// sploosh After Refactor using if
// Gnuplot not found, using plotters backend
// sploosh                 time:   [1.1701 ns 1.2000 ns 1.2509 ns]
//                         change: [+1.1023% +2.1203% +3.7991%] (p = 0.00 < 0.05)
//                         Performance has regressed.
// Found 10 outliers among 100 measurements (10.00%)
//   6 (6.00%) high mild
//   4 (4.00%) high severe

// sploosh After Refactor using under_score
// Gnuplot not found, using plotters backend
// sploosh                 time:   [1.1757 ns 1.1801 ns 1.1846 ns]
//                         change: [-1.3713% +0.3109% +1.6030%] (p = 0.74 > 0.05)
//                         No change in performance detected.
// Found 1 outliers among 100 measurements (1.00%)
//   1 (1.00%) high severe

// splish After Refactor
// Gnuplot not found, using plotters backend
// sploosh                 time:   [1.1511 ns 1.1532 ns 1.1559 ns]
//                         change: [-2.6973% -2.0449% -1.5133%] (p = 0.00 < 0.05)
//                         Performance has improved.
// Found 4 outliers among 100 measurements (4.00%)
//   1 (1.00%) high mild
//   3 (3.00%) high severe
