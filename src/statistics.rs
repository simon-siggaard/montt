use puruspe::inverf;
use rand_distr::LogNormal;

pub fn log_normal_from_estimates(q50: f64, q95: f64) -> LogNormal<f64> {
    let sigma = 1.0 / 2.0_f64.sqrt()
        * ((q95 / q50).ln() / (inverf(2.0 * 0.95 - 1.0) - inverf(2.0 * 0.50 - 1.0)));
    let mu = q50.ln() - (2.0 * sigma * sigma).sqrt() * inverf(2.0 * 0.50 - 1.0);
    let mean = (mu + 0.5 * sigma * sigma).exp();

    LogNormal::from_mean_cv(mean, sigma).unwrap()
}
