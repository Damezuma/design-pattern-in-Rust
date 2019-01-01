enum DirectoryItem{
    Directory{
        name:String,
        children:Vec<DirectoryItem>
    },
    File{
        name:String,
        size:usize
    }
}
impl DirectoryItem{
    fn dir<NAME>(name:NAME, children:Vec<Self>)->Self
    where NAME:AsRef<str>{
        DirectoryItem::Directory{
            name:name.as_ref().to_string(),
            children:children
        }
    }
    fn file<NAME>(name:NAME, size:usize)->Self
    where NAME:AsRef<str>{
        DirectoryItem::File{
            name:name.as_ref().to_string(),
            size:size
        }
    }
    fn print(&self){
        Self::print_el(self, 0);
    }
    fn print_el(item:&DirectoryItem ,  step:u32){
        let mathod = match item{
            DirectoryItem::Directory{..}=>Self::print_dir,
            DirectoryItem::File{..}=>Self::print_file
        };
        mathod(item, step);
    }
    fn print_dir(item:&DirectoryItem, step:u32){
        let (chilren, name) = if let DirectoryItem::Directory{children, name} = item{
            (children, name)
        }
        else{
            unimplemented!();
        };
        if step != 0{
            for _ in 1..step{
                print!("│  ");
            }
            print!("├──");
        }
        println!("{}", name);
        for item in chilren{
            Self::print_el(item, step + 1);
        }
    }
    fn print_file(item:&DirectoryItem, step:u32){
        let (name, size) = if let DirectoryItem::File{name, size} = item{
            (name, size)
        }
        else{
            unimplemented!();
        };
        
        if step != 1{
            for _ in 1..step{
                print!("│  ");
            }
        }
        print!("├──");
        println!("{}(size:{})", name, size);
    }
}
fn main(){
    let tree = DirectoryItem::dir("root", vec![
       DirectoryItem::file("A", 500), 
       DirectoryItem::file("B", 500),
       DirectoryItem::dir("d-1", vec![
            DirectoryItem::file("A", 500),
            DirectoryItem::dir("d-2", vec![
                DirectoryItem::file("A", 500), 
                DirectoryItem::file("B", 500)
            ]),
            DirectoryItem::file("B", 500)
       ]),
       DirectoryItem::file("C", 4500),
    ]);
    tree.print();
}
