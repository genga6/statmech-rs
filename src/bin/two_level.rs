use rand::prelude::*;

fn energy(s: i32, h: f64) -> f64 {
    -h * s as f64
}

fn sample_magnetization(beta: f64, h: f64, nsamples: usize, rng: &mut impl Rng) -> (f64, f64) {
    // 直接サンプリング
    let w_up = (-beta * energy(1, h)).exp();
    let w_dn = (-beta * energy(-1, h)).exp();
    let z = w_up + w_dn; // 分配関数
    let p_up = w_up / z; // s = +1 を引く確率

    let mut sum = 0.0;
    let mut sum_sq = 0.0;

    for _ in 0..nsamples {
        let s = if rng.random_bool(p_up) { 1 } else { -1 };
        sum += s as f64;
        sum_sq += (s * s) as f64;
    }

    let n = nsamples as f64;
    let mean = sum / n;
    let var = sum_sq / n - mean * mean; // 標本分散
    let stderr = (var / n).sqrt();
    (mean, stderr)
}

fn main() {
    let h = 1.0;
    let nsamples = 100_000;

    let mut rng = rand::rngs::StdRng::seed_from_u64(20260829);

    println!("  beta        <s>_MC      stderr      tanh(bh)    ずれ/σ");
    for i in 1..=10 {
        let beta = 0.2 * i as f64;
        let (mean, stderr) = sample_magnetization(beta, h, nsamples, &mut rng); // 直接サンプリング
        let exact = (beta * h).tanh(); // スピンの期待値の厳密解
        let z_score = (mean - exact) / stderr; // error bar
        println!("{beta:6.2}  {mean:10.6}  {stderr:8.6}  {exact:10.6}  {z_score:+8.2}");
    }
}
