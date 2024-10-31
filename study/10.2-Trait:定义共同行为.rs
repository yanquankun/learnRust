mod anmail_module {
    // trait 类似typescript中的interface
    // 负责描述  不负责具体实现
    pub trait Animal {
        fn eat(&self) -> String;

        fn walk(&self, speed: &str) -> String;

        fn noise(&self, noice: &str) -> String {
            // 可返回默认值
            format!("animal make noise: {} , {}", noice, self.eat())
        }
    }

    pub struct Dog {
        pub name: String,
    }
    pub struct Cat {
        pub name: String,
    }
    // 通过impl为子类继承父类属性，类似typescript中的extends
    impl Animal for Dog {
        // fn noise(&self) -> String {
        //     format!("{}: wang!", self.name)
        // }
        //
        // noise中包含了eat的引用，此处只需要实现eat即可
        fn eat(&self) -> String {
            format!("{} 吃狗粮!", self.name)
        }

        fn walk(&self, speed: &str) -> String {
            format!("{}: 骑车{}!", self.name, speed)
        }
    }
    impl Animal for Cat {
        // fn noise(&self) -> String {
        //     format!("{}: miao!", self.name)
        // }
        fn eat(&self) -> String {
            format!("{} 吃猫粮!", self.name)
        }

        fn walk(&self, speed: &str) -> String {
            format!("{}: 骑车{}!", self.name, speed)
        }
    }

    // trait可作为参数使用，借用实现了Animal的方法
    // 实现了多态
    pub fn _say(animal: &impl Animal) {
        println!("我们是动物：我是{}", animal.eat());
    }
    // Trait Bound 语法
    pub fn say<T: Animal>(animal: &T) {
        println!("我是动物：我是{}", animal.eat());
    }
}
pub trait Animal2 {
    fn eat2(&self) -> String;
}
fn main() {
    use anmail_module::{say, Animal, Cat, Dog};
    let dog = Dog {
        name: String::from("旺财"),
    };
    let cat = Cat {
        name: String::from("咪咪"),
    };
    println!("{}", dog.noise("汪汪"));
    println!("{}", cat.noise("喵喵"));
    println!("{}", dog.walk("快"));
    println!("{}", cat.walk("慢"));
    say(&dog);

    // trait 可以联合使用
    fn _all(item: &(impl Animal + Animal2)) {
        println!("Animal:{}, Animal2:{}", item.eat(), item.eat2());
    }
    fn _all2<T: Animal + Animal2>(item: &T) {
        println!("Animal:{}, Animal2:{}", item.eat(), item.eat2());
    }
}
