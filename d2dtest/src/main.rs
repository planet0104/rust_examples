
mod windows;
use windows::*;
use std::io::Result;
struct Game{
    red: i32,
    dir: i32,
}

impl State for Game{
    fn new(window:&mut Window) -> Self{
        window.load_assets(vec![
            (String::from("bird.png"), AssetsType::Image),
            (String::from("bird.png"), AssetsType::File)
        ]);
        Game{
            red: 0,
            dir: 1
        }
    }

    fn update(&mut self, window:&mut Window){
        if self.red == 255{
            self.dir = -1;
        }
        if self.red == 0{
            self.dir = 1;
        }
        self.red += self.dir;
    }
    fn draw(&mut self, graphics:&mut Graphics, window:&mut Window){
        graphics.clear(self.red as u8, 0, 0);
    }

    fn on_assets_load(&mut self, path: &str, assets:Result<Assets>, window: &mut Window){
        match assets{
            Err(err) => println!("资源加载失败：{:?}", err),
            Ok(Assets::File(data)) => println!("文件加载成功：{:?}", data.len()),
            Ok(Assets::Image(bmp)) => println!("图片加载成功：{:?}", bmp.size()),
        };
    }
}

fn main(){
    windows::run::<Game>();
}