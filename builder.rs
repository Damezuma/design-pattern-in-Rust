struct WindowBuilder{
    _width:Option<usize>,
    _height:Option<usize>,
    _has_border:Option<usize>,
    _title:Option<String>
}
impl WindowBuilder{
    fn new()->Self{
        Self{
            _width:None,
            _height:None,
            _has_border:None,
            _title:None
        }
    }
    fn width(mut self, val:usize)->Self{
        self._width = Some(val);
        self
    }
    fn height(mut self, val:usize)->Self{
        self._height = Some(val);
        self
    }
    fn border(mut self, val:usize)->Self{
        self._has_border = Some(val);
        self
    }
    fn title<STR>(mut self, val:STR)->Self
    where STR:AsRef<str>{
        self._title = Some(String::from(val.as_ref()));
        self
    }
    fn build(self)->Window{
        Window{
            width:self._width.unwrap_or(300usize),
            height:self._height.unwrap_or(300usize),
            border:self._has_border.unwrap_or(1usize),
            title:self._title.unwrap_or("hello, world".to_string())
        }
    }
}
struct Window{
    width:usize,
    height:usize,
    border:usize,
    title:String
}
impl Window{
    fn print(&self){
        println!("title: {}, w:{}, h:{}, border:{} thickness",
        self.title,
        self.width,
        self.height,
        self.border);
    }
}
fn main() {
    let window = WindowBuilder::new()
    .title("hello, world").build();
    window.print();
}
