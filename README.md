# kinds
Higher-Kinded Types in Rust by GAT

## Example
```rust
use kinds::kind::*;

enum Nat<'a, K: Kind<'a> + 'a> {
    Zero,
    Next(K::F<Nat<'a, K>>),
}

type BoxedNats = Box<Nat<'static, kinds::std::boxed::Box>>;

fn sum_boxed_nats(n: &BoxedNats) -> (usize, usize) {
    let n = n.as_ref();
    match n {
        Nat::Zero => (0, 0),
        Nat::Next(n) => {
            let (sum, n) = sum_boxed_nats(n);
            (sum + n + 1, n + 1)
        }
    }
}

#[test]
fn test_sum_nats() {
    let n: BoxedNats = Box::new(Nat::Next(Box::new(Nat::Next(Box::new(Nat::Next(
        Box::new(Nat::Zero),
    ))))));
    let sum = sum_boxed_nats(&n).0;
    assert_eq!(sum, 6);
}

```