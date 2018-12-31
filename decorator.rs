trait Decorator{
    fn run(&self, text:String)->String;
}
struct A<T:Decorator>{
    base:T,
    process:RefCell<Box<FnMut(String)->String>>
}
struct B<T:Decorator + Sized>{
    base:T
}
impl<T:Decorator +Sized> A<T>{
    fn new<F>(base:T, process:F)->Self
    where F:'static + FnMut(String)->String{
        Self{
            base:base,
            process:RefCell::new(Box::new( process))
        }   
    }
}
impl<T:Decorator + Sized> Decorator for A<T>{
    fn run(&self, text:String)->String{
        let mut x =  self.process.borrow_mut();
        let f:&mut dyn FnMut(String)->String = x.as_mut();
        let text = self.base.run(text);
        f(text)
    }
}
impl<T:Decorator + Sized> B<T>{
    fn new(base:T)->Self{
        Self{
            base:base
        }
    }
}
impl<T:Decorator + Sized> Decorator for B<T>{
    fn run(&self, text:String)->String{
        let text = self.base.run(text);
        text + " with chino, "
    }
}
struct Root;
impl Root{
    fn new()->Self{
        Self{}
    }
}
impl Decorator for Root{
    fn run(&self, text:String)->String{
        text
    }
}
fn main() {
    let a = B::new(
        A::new(
            Root::new(),
            |text| text + " with cocoa"
        )
    );
    println!("{}",a.run("gochiusa".to_string()));
}
