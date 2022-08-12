//! Module containing unit tests.
#![cfg(test)]

use uom::si::f64::*;

#[test]
fn test_unit() {
    let time: Time = Time::new::<uom::si::time::second>(2.0);
    let charge: ElectricCharge = ElectricCharge::new::<uom::si::electric_charge::coulomb>(3.0);
    let amps: ElectricCurrent = charge / time;
    assert_eq!(ElectricCurrent::new::<uom::si::electric_current::ampere>(1.5), amps);

    let volts: ElectricPotential = ElectricPotential::new::<uom::si::electric_potential::volt>(5.0);
    let power: Power = volts * amps;
    assert_eq!(Power::new::<uom::si::power::watt>(7.5), power);
}
