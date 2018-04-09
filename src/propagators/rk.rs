pub use super::RK;

/// `CashKarp45` is a [Runge Kutta Cash Karp integrator](https://en.wikipedia.org/wiki/Cash%E2%80%93Karp_method).
pub struct CashKarp45 {}

impl RK for CashKarp45 {
    fn order() -> u8 {
        5
    }
    fn stages() -> usize {
        6
    }
    fn a_coeffs() -> &'static [f64] {
        &[
            1.0 / 5.0,
            3.0 / 40.0,
            9.0 / 40.0,
            3.0 / 10.0,
            -9.0 / 10.0,
            6.0 / 5.0,
            -11.0 / 54.0,
            5.0 / 2.0,
            -70.0 / 27.0,
            35.0 / 27.0,
            1631.0 / 55296.0,
            175.0 / 512.0,
            575.0 / 13824.0,
            44275.0 / 110592.0,
            253.0 / 4096.0,
        ]
    }
    fn b_coeffs() -> &'static [f64] {
        &[
            37.0 / 378.0,
            0.0,
            250.0 / 621.0,
            125.0 / 594.0,
            0.0,
            512.0 / 1771.0,
            2825.0 / 27648.0,
            0.0,
            18575.0 / 48384.0,
            13525.0 / 55296.0,
            277.0 / 14336.0,
            1.0 / 4.0,
        ]
    }
}

/// `RK4Fixed` is a fixed step RK4. If initialized with an `Options.with_adaptive_step`, the variable step
/// will **not** be taken into consideration.
pub struct RK4Fixed {}

impl RK for RK4Fixed {
    fn order() -> u8 {
        4
    }
    fn stages() -> usize {
        4
    }
    fn a_coeffs() -> &'static [f64] {
        &[0.5, 0.0, 0.5, 0.0, 0.0, 1.0]
    }
    fn b_coeffs() -> &'static [f64] {
        &[
            1.0 / 6.0,
            1.0 / 3.0,
            1.0 / 3.0,
            1.0 / 6.0,
            // NOTE: Duplicating the B coefficients for force the error to zero.
            1.0 / 6.0,
            1.0 / 3.0,
            1.0 / 3.0,
            1.0 / 6.0,
        ]
    }
}

const SQRT6: f64 = 2.449489742783178;

/// `RK89` is a Runge Kutta 8-9 integrator. Coefficients taken from GMAT `src/base/propagator/RungeKutta89.cpp`.
pub struct RK89 {}

impl RK for RK89 {
    fn order() -> u8 {
        9
    }
    fn stages() -> usize {
        16
    }
    fn a_coeffs() -> &'static [f64] {
        &[
            1.0 / 12.0,
            1.0 / 27.0,
            2.0 / 27.0,
            1.0 / 24.0,
            0.0,
            1.0 / 8.0,
            (4.0 + 94.0 * SQRT6) / 375.0,
            0.0,
            (-94.0 - 84.0 * SQRT6) / 125.0,
            (328.0 + 208.0 * SQRT6) / 375.0,
            (9.0 - SQRT6) / 150.0,
            0.0,
            0.0,
            (312.0 + 32.0 * SQRT6) / 1425.0,
            (69.0 + 29.0 * SQRT6) / 570.0,
            (927.0 - 347.0 * SQRT6) / 1250.0,
            0.0,
            0.0,
            (-16248.0 + 7328.0 * SQRT6) / 9375.0,
            (-489.0 + 179.0 * SQRT6) / 3750.0,
            (14268.0 - 5798.0 * SQRT6) / 9375.0,
            2.0 / 27.0,
            0.0,
            0.0,
            0.0,
            0.0,
            (16.0 - SQRT6) / 54.0,
            (16.0 + SQRT6) / 54.0,
            19.0 / 256.0,
            0.0,
            0.0,
            0.0,
            0.0,
            (118.0 - 23.0 * SQRT6) / 512.0,
            (118.0 + 23.0 * SQRT6) / 512.0,
            -9.0 / 256.0,
            11.0 / 144.0,
            0.0,
            0.0,
            0.0,
            0.0,
            (266.0 - SQRT6) / 864.0,
            (266.0 + SQRT6) / 864.0,
            -1.0 / 16.0,
            -8.0 / 27.0,
            (5034.0 - 271.0 * SQRT6) / 61440.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            (7859.0 - 1626.0 * SQRT6) / 10240.0,
            (-2232.0 + 813.0 * SQRT6) / 20480.0,
            (-594.0 + 271.0 * SQRT6) / 960.0,
            (657.0 - 813.0 * SQRT6) / 5120.0,
            (5996.0 - 3794.0 * SQRT6) / 405.0,
            0.0,
            0.0,
            0.0,
            0.0,
            (-4342.0 - 338.0 * SQRT6) / 9.0,
            (154922.0 - 40458.0 * SQRT6) / 135.0,
            (-4176.0 + 3794.0 * SQRT6) / 45.0,
            (-340864.0 + 242816.0 * SQRT6) / 405.0,
            (26304.0 - 15176.0 * SQRT6) / 45.0,
            -26624.0 / 81.0,
            (3793.0 + 2168.0 * SQRT6) / 103680.0,
            0.0,
            0.0,
            0.0,
            0.0,
            (4042.0 + 2263.0 * SQRT6) / 13824.0,
            (-231278.0 + 40717.0 * SQRT6) / 69120.0,
            (7947.0 - 2168.0 * SQRT6) / 11520.0,
            (1048.0 - 542.0 * SQRT6) / 405.0,
            (-1383.0 + 542.0 * SQRT6) / 720.0,
            2624.0 / 1053.0,
            3.0 / 1664.0,
            -137.0 / 1296.0,
            0.0,
            0.0,
            0.0,
            0.0,
            (5642.0 - 337.0 * SQRT6) / 864.0,
            (5642.0 + 337.0 * SQRT6) / 864.0,
            -299.0 / 48.0,
            184.0 / 81.0,
            -44.0 / 9.0,
            -5120.0 / 1053.0,
            -11.0 / 468.0,
            16.0 / 9.0,
            (33617.0 - 2168.0 * SQRT6) / 518400.0,
            0.0,
            0.0,
            0.0,
            0.0,
            (-3846.0 + 31.0 * SQRT6) / 13824.0,
            (155338.0 - 52807.0 * SQRT6) / 345600.0,
            (-12537.0 + 2168.0 * SQRT6) / 57600.0,
            (92.0 + 542.0 * SQRT6) / 2025.0,
            (-1797.0 - 542.0 * SQRT6) / 3600.0,
            320.0 / 567.0,
            -1.0 / 1920.0,
            4.0 / 105.0,
            0.0,
            (-36487.0 - 30352.0 * SQRT6) / 279600.0,
            0.0,
            0.0,
            0.0,
            0.0,
            (-29666.0 - 4499.0 * SQRT6) / 7456.0,
            (2779182.0 - 615973.0 * SQRT6) / 186400.0,
            (-94329.0 + 91056.0 * SQRT6) / 93200.0,
            (-232192.0 + 121408.0 * SQRT6) / 17475.0,
            (101226.0 - 22764.0 * SQRT6) / 5825.0,
            -169984.0 / 9087.0,
            -87.0 / 30290.0,
            492.0 / 1165.0,
            0.0,
            1260.0 / 233.0,
        ]
    }
    fn b_coeffs() -> &'static [f64] {
        &[
            23.0 / 525.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            171.0 / 1400.0,
            86.0 / 525.0,
            93.0 / 280.0,
            -2048.0 / 6825.0,
            -3.0 / 18200.0,
            39.0 / 175.0,
            0.0,
            9.0 / 25.0,
            233.0 / 4200.0,
            -7.0 / 400.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            63.0 / 200.0,
            -14.0 / 25.0,
            21.0 / 20.0,
            -1024.0 / 975.0,
            -21.0 / 36400.0,
            -3.0 / 25.0,
            -9.0 / 280.0,
            9.0 / 25.0,
            233.0 / 4200.0,
        ]
    }
}
