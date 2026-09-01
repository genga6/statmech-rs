//! 2 次元正方格子 Ising 模型(強磁性·周期境界条件)。
//!
//!     H= -J Σ_<ij> s_is_j,  s_i = ±1
//!
//! 以下 J = 1 , k_B = 1と取る。したがって β = 1/T。
//! Onsager の厳密解による臨界温度は T_c = 2 / ln(1 + √2) ~ 2.269。

use rand::prelude::*;

// Onsager の臨界温度 T_c = 2 / ln(1 + √2)
pub const T_C: f64 = 2.0 / (1.0 + 2.0_f64.sqrt()).ln();

// L × L 正方格子状の Ising モデル
pub struct Ising2d {
    l: usize,
    spins: Vec<i8>, // (i, j) のスピンは spins[i * l + j]
}
