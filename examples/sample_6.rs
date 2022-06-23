use std::cell::RefCell;

struct B {
    c: char,
    s: RefCell<String>,
}

fn main() {
    let b = B {c: 'a', s: RefCell::new("alex".to_string())};

    let rb = &b;
    // 可変参照を取って'a'を追加する
    rb.s.borrow_mut().push('a');

    {
        // 不変参照
        let rbs = b.s.borrow();
        assert_eq!(&*rbs, "alexa");

        // 可変参照を取ろうとするがエラー
        b.s.borrow_mut();
        assert!(b.s.try_borrow_mut().is_err());
    }

    assert!(b.s.try_borrow_mut().is_ok());
}
