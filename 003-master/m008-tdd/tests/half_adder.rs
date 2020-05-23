use logic_gates::{and, xor};

pub type Sum = u8;
pub type Carry = u8;

pub fn half_adder_io() -> Vec<((u8, u8), (Sum, Carry))> {
    vec![
        ((0, 0), (0, 0)),
        ((1, 0), (1, 0)),
        ((0, 1), (1, 0)),
        ((1, 1), (0, 1)),
    ]
}

/// This function implements a half adder using primitive gates
fn half_adder (a: u8, b: u8) -> (Sum, Carry) {
    (xor(a, b), and (a, b))
}

#[test]
fn one_bit_adder() {
    for (i, o) in half_adder_io(){
        let (a, b) = i;
        //println!("Testing: {}, {} -> {}", a, b, o);
        assert_eq!(half_adder(a, b), o);
    }
}
