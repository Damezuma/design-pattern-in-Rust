trait Chicken{
    fn preprocess(&mut self);
    fn cook(&mut self);
    fn show_me(&self);
}
trait ChickenFactory{
    type Return:Chicken;
    fn create_chicken(&self)->Self::Return;
}

struct FriedChicken;
impl FriedChicken{
    fn new()->Self{
        Self{}
    }
}
impl Chicken for FriedChicken{
    fn preprocess(&mut self){
        println!("닭을 손질합니다!");
    }
    fn cook(&mut self){
        println!("닭을 튀깁니다!");
    }
    fn show_me(&self){
        println!("난 프라이드 치킨!");
    }
}
struct YangnyumChicken;
impl YangnyumChicken{
    fn new()->Self{
        Self{}
    }
}
impl Chicken for YangnyumChicken{
    fn preprocess(&mut self){
        println!("닭을 손질합니다!");
        println!("양념을 준비합니다!");
    }
    fn cook(&mut self){
        println!("닭을 튀깁니다!");
        println!("준비한 양념에 튀긴 닭을 버무립니다!");
    }
    fn show_me(&self){
        println!("난 양념 치킨!");
    }
}
struct FriedChickenFactory{

}
impl FriedChickenFactory{
    fn new()->Self{
        Self{}
    }
}
impl ChickenFactory for FriedChickenFactory{
    type Return = FriedChicken;
    fn create_chicken(&self)->Self::Return{
        Self::Return::new()
    }
}

struct YangnyumChickenFactory{

}
impl YangnyumChickenFactory{
    fn new()->Self{
        Self{}
    }
}
impl ChickenFactory for YangnyumChickenFactory{
    type Return = YangnyumChicken;
    fn create_chicken(&self)->Self::Return{
        Self::Return::new()
    }
}
struct ChickenStore<Factory:ChickenFactory>{
    factory:Factory
}
impl<Factory> ChickenStore<Factory>
where Factory:ChickenFactory{
    fn new(factory:Factory)->Self{
        Self{factory:factory}
    }
    fn order_chicken(&self)->impl Chicken{
        let mut chicken = self.factory.create_chicken();
        chicken.preprocess();
        chicken.cook();
        chicken
    }
}

fn main(){
    let store = ChickenStore::new(YangnyumChickenFactory::new());
    let chicken = store.order_chicken();
    chicken.show_me();
    let store = ChickenStore::new(FriedChickenFactory::new());
    let chicken = store.order_chicken();
    chicken.show_me();
}
