// 構造体を定義する
// println!の"{:?}"で表示できるようにDebugトレイとを自動導出しておく
#[derive(Debug)]
pub struct Parent(pub usize, pub Child, pub Child); // Parentはusizeに加えてChildを2つ持つ

#[derive(Debug)]
pub struct Child(pub usize);

use std::ops::Drop;

// Parent構造体にデストラクタを実装する
impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}
// Child構造体にデストラクタを実装する
impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

pub fn ch_main() {
    let p1 = Parent(1, Child(11), Child(12));

    {
        let p2 = Parent(2, Child(21), Child(22));
        println!("(a) p1: {:?}, p2: {:?}", p1, p2);
    }
    println!("(b) p1: {:?}", p1);

    let p3 = Parent(3, Child(31), Child(32));
    println!("(a) p1: {:?}, p3: {:?}", p1, p3);
}
