
use super::ch07_01_value_scop::Parent;
use super::ch07_01_value_scop::Child;

// Parentへの不変の参照を引数にとる
fn f1(p: &Parent) {
    println!("p: {:?}", p);
}

// Parentへの可変の参照を引数にとる
fn f2(p: &mut Parent) {
    p.0 *= -1;
}

pub fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));

    let p2 = p1;

    println!("p2: {:?}", p2);
    // println!("p1: {:?}", p1); // p1は値の所有権を失ったのでアクセス不可
    // -> error[E0382]: borrow of moved value: p1

    p1 = Parent(2, Child(21), Child(22));
    println!("p1: {:?}", p1); // p1は別の値の所有権を持つのでアクセスできる

    let mut p3 = Parent(3, Child(31), Child(32));
    f1(&p3); // f1には所有権をムーブせず、不変の参照を渡す
    f2(&mut p3); // f2には所有権をムーブせず、可変の参照を渡す
    println!("p3: {:?}", p3);
}