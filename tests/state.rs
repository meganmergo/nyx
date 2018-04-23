extern crate nalgebra as na;
extern crate nyx_space as nyx;

#[test]
fn state_def_leo() {
    use nyx::celestia::{State, EARTH};
    let leo =
        State::from_cartesian::<EARTH>(-2436.45, -2436.45, 6891.037, 5.088611, -5.088611, 0.0);
    assert_eq!(leo.x, -2436.45, "x");
    assert_eq!(leo.y, -2436.45, "y");
    assert_eq!(leo.z, 6891.037, "z");
    assert_eq!(leo.vx, 5.088611, "vx");
    assert_eq!(leo.vy, -5.088611, "vy");
    assert_eq!(leo.vz, 0.0, "vz");
    assert_eq!(leo.energy(), -25.842247282849137, "energy");
    assert_eq!(leo.period(), 6740.269063643045, "period");
    assert_eq!(leo.sma(), 7712.186117895043, "sma");
    assert_eq!(leo.inc(), 63.43400340775114, "inc");
    assert_eq!(leo.aop(), 135.0, "aop");
    // assert_eq!(leo.ta(), 90.0, "ta");
    assert_eq!(leo.ecc(), 0.0009995828314320525, "ecc"); // XXX: Error is 1e-3!! Must check how it's computed in GMAT

    //-25.84224728284914        6740.269063643045             7712.186117895043         0.0009995828314320525     63.43400340775114             135                            90                            0
}
