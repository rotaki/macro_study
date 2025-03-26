fn compose_two<FIRST, SECOND, THIRD, F, G>(f: F, g: G) -> impl Fn(FIRST) -> THIRD
where
    F: Fn(FIRST) -> SECOND,
    G: Fn(SECOND) -> THIRD,
{
    move |x| g(f(x))
}

macro_rules! compose_naive {
    ($last:expr) => { $last };
    ($head:expr , $($tail:expr),+) => {
        compose_two($head, compose_naive!($($tail),+))
    }
}

macro_rules! compose_alt {
    ($last:expr) => { $last };
    ($head:expr => $($tail:expr)=>+) => {
        compose_two($head, compose_alt!($($tail)=>+))
    }
}

macro_rules! compose {
    ($($f:expr),+) => {
        |x| {
            let mut val = x;
            $(
                val = $f(val);
            )+
            val
        }
    }
}

fn main() {
    let f = |x| x + 1;
    let g = |x| x * 2;
    let h = compose_two(compose_two(compose_two(f, g), f), g);

    println!("{}", h(1));

    let h = compose_naive!(f, g, f, g);
    println!("{}", h(1));

    let h = compose_alt!(f => g => f => g);
    println!("{}", h(1));

    let h = compose!(f, g, f, g);
    println!("{}", h(1));
}
