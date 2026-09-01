use rand::prelude::*;

fn energy(s: i32, h: f64) -> f64 {
    -(h * s as f64)
}

fn metropolis_step(s: &mut i32, beta: f64, h: f64, rng: &mut impl Rng) -> bool {
    let s_new = -*s;
    let delta_e = energy(s_new, h) - energy(*s, h); // ΔE = E_new - E_old

    // ΔE <= 0 なら必ず受理
    if delta_e <= 0.0 || rng.random::<f64>() < (-beta * delta_e).exp() {
        *s = s_new;
        true
    } else {
        false
    }
}

fn binned_stderr(samples: &[f64], bin_size: usize) -> f64 {
    let nbins = samples.len() / bin_size;
    let block: Vec<f64> = (0..nbins)
        .map(|b| {
            let chunk = &samples[b * bin_size..(b + 1) * bin_size];
            chunk.iter().sum::<f64>() / bin_size as f64
        })
        .collect();
    let mean = block.iter().sum::<f64>() / nbins as f64;
    let var = block.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (nbins - 1) as f64;
    (var / nbins as f64).sqrt()
}

fn main() {
    let h = 1.0;
    let nsteps = 100_000;
    let nburn = 1_000; // シミュレーションが熱化するまでのステップ数
    let bin_size = 100;

    let mut rng = rand::rngs::StdRng::seed_from_u64(20260831);

    println!(" beta  受理率  <s>_MC  素の誤差  ビン誤差    比  tanh(bh)    ずれ/σ");

    for i in 1..=10 {
        let beta = 0.2 * i as f64;
        let mut s = 1;

        // 助走
        for _ in 0..nburn {
            metropolis_step(&mut s, beta, h, &mut rng);
        }

        // 測定
        let mut samples = Vec::with_capacity(nsteps);
        let mut naccept = 0usize;

        for _ in 0..nsteps {
            if metropolis_step(&mut s, beta, h, &mut rng) {
                naccept += 1;
            }
            samples.push(s as f64);
        }

        let n = nsteps as f64;
        let mean = samples.iter().sum::<f64>() / n;
        let var = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
        let err_naive = (var / n).sqrt();
        let err_bin = binned_stderr(&samples, bin_size);

        let exact = (beta * h).tanh();
        let z = (mean - exact) / err_bin;
        let accept_rate = naccept as f64 / n;
        let ratio = err_bin / err_naive;

        println!(
            "{beta:5.2} {accept_rate:6.4} {mean:9.6} {err_naive:9.6} {err_bin:9.6} {ratio:5.2} {exact:9.6} {z:+7.2}"
        );
    }
}
