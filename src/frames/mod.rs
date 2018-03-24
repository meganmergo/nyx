extern crate nalgebra as na;

use na::Vector6;
use std::fmt::Display;

/// The `Orbit` trait is common to all orbit definitions. Each orbit definition however may define
/// additional constructors. However, these constructors are necessary to ensure that the dynamics
/// module will support propagating specific orbital element defitions.
#[derive(Copy)]
pub trait Orbit {
    /// Returns the gravitational parameter (mu) associated with this orbit.
    fn gm() -> f64;
    /// Returns this orbit as a Vector6 of f64. All orbits can be representated as 6 elements.
    /// This is used for compatibility with the `propagator` module.
    fn to_state() -> Vector6<f64>;
    /// Creates a new type from this state. This is use for  compatibility with the `propagator` module.
    // XXX: Should this be an update(s: Vector6)? That way the frame is implied.
    fn from_state(s: Vector6<f64>) -> Self;
    /// Borrow the frame in which this orbit is defined.
    fn frame(&self) -> &Frame;
}

/// The `Frame` trait defines a reference frame.
#[derive(Copy)]
pub trait Frame: Display {
    /// Borrows whether or not this frame is inertial or not.
    fn inertial(&self) -> bool;
    /// Borrows the origin of this frame.
    fn origin(&self) -> &Origin;
}

#[derive(Debug)]
pub enum Origin {
    Sun,
    Mercury, Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

impl Origin {
    // XXX: Figure out a way to modify these values if an SPK is loaded.
    // Returns the default gravitational paramater (mu) of the given origin in km.N/s^2 (XXX: unit?)
    fn gm(&self) -> f64 {
        match *self {
            Origin::Sun => 1.32712440017987e11,
            Origin::Mercury => 2.2032e4,
            Origin::Venus => 3.24858599e5,
            Origns::Earth => 3.98600433e5,
            Origin::Mars => 4.28283100e4,
            Origin::Jupiter => 1.266865361e8,
            Origin::Saturn => 3.7931208e7,
            Origin::Uranus => 5.7939513e6,
            Origin::Neptune => 6.8365299e6,
        }
    }

    // Returns the default radius of the given origin in kilometers
    fn radius(&self) -> f64 {
        match *self {
            Origin::Sun => 695_700.0,
            Origin::Mercury => 2_439.7,
            Origin::Venus => 6_051.8,
            Origns::Earth => 6_378.1363,
            Origin::Mars => 3_396.19,
            Origin::Jupiter => 71_492.0,
            Origin::Saturn => 60_268.0,
            Origin::Uranus => 25_559.0,
            Origin::Neptune => 24_622.0,
        }
    }
}
