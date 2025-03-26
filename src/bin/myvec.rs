macro_rules! myvec {
    () => (
        Vec::new()
    );
    (make an empty vec) => (
        Vec::new()
    );
    ($x:expr) => (
        {
            let mut v = Vec::new();
            v.push($x);
            v
        }
    );
    ($($x:expr),*) => (
        {
            let mut v = Vec::new();
            $(v.push($x);)*
            v
        }
    );
}

fn main() {
    // let mut v = myvec!();
    // let mut v = myvec!(make an empty vec);
    // let mut v = myvec!(2);
    let mut v = myvec!(1, 2, 3);
    v.push(1);
    println!("{:?}", v);
}
