use std::cell::RefCell;
use std::collections::HashSet;

fn main() {
    thread_local!(
        static RABBITS: RefCell<HashSet<&'static str>> = {
            let rb = ["ロップイヤー", "ダッチ"].iter().cloned().collect();
            RefCell::new(rb)
        }
    );

    // mainスレッド
    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ロップイヤー"));
        rb.borrow_mut().insert("ネザーランド・ドワーフ");
    });

    // 別スレッドを作成して、TLSにドワーフホトを追加
    std::thread::spawn(||
        RABBITS.with(|rb| rb.borrow_mut().insert("ドワーフホト"))
    ).join().expect("Thread error");

    // mmainスレッドを確認。別スレッドに追加したドワーフホトが反映されないことを確認
    RABBITS.with(|rb| {
        assert!(rb.borrow().contains("ネザーランド・ドワーフ"));
        assert!(!rb.borrow().contains("ドワーフホト"));
    })
}