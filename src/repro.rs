use rquickjs::{Context, Runtime, bind};

#[bind(object)]
#[quickjs(bare)]
mod geom {
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    impl Point {
        // constructor
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        // instance method
        pub fn test(&self) -> f64 {
            self.x + self.y
        }
    }
}

fn main() {
    let runtime = Runtime::new().unwrap();

    println!("First context");
    let ctx = Context::full(&runtime).unwrap();
    ctx.with(|ctx| {
        ctx.globals().init_def::<Geom>().unwrap();
    });

    println!("Second context");
    let ctx2 = Context::full(&runtime).unwrap();
    ctx2.with(|ctx| {
        ctx.globals().init_def::<Geom>().unwrap();
    });
}
