extern crate orderless;
use orderless::orderless;

fn main() {
    let xs = (1, 3, 2);

    match xs {
        orderless!((1, 2, 3)) => {
            println!("matched {xs:?}!")
        }
        _ => {
            println!("not matched {xs:?} :<")
        }
    }
}

#[test]
fn all_permutations() {
    assert!(match (10, 20, 30) { orderless!((10, 20, 30)) => true, _ => false });
    assert!(match (10, 30, 20) { orderless!((10, 20, 30)) => true, _ => false });
    assert!(match (20, 10, 30) { orderless!((10, 20, 30)) => true, _ => false });
    assert!(match (20, 30, 10) { orderless!((10, 20, 30)) => true, _ => false });
    assert!(match (30, 10, 20) { orderless!((10, 20, 30)) => true, _ => false });
    assert!(match (30, 20, 10) { orderless!((10, 20, 30)) => true, _ => false });
}

#[test]
fn stress_testing() {
    assert!(match (1, 2, 3, 4, 5, 6, 7) { orderless!((7, 6, 5, 4, 3, 2, 1)) => true, _ => false });
}
