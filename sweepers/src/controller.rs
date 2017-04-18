extern crate gdi32;
extern crate user32;
extern crate winapi;
use winapi::windef::{ HGDIOBJ, HPEN, HDC, COLORREF };
use winapi::wingdi::{ PS_SOLID };
use params::{ NUM_ELITE, NUM_TICKS, MINE_SCALE, WINDOW_WIDTH, WINDOW_HEIGHT, NUM_MINES, NUM_SWEEPERS, MUTATION_RATE, CROSSOVER_RATE };
use std::ptr::null_mut;

//控制器
use gen_alg::{ Genome };
use mine_sweeper::{ MineSweeper };
use vector_2d::{Vector2D};
use gen_alg::{ GenAlg };
use matrix::Matrix;
use utils::{ random_float, PointF, rgb };
use devices::{ str_to_ws,move_to_ex, line_to, text_out };

const NUM_SWEEPER_VERTS :usize = 16;
const NUM_MINE_VERTS :usize = 4;

pub struct Controller {
    //存储基因组群体
    the_population: Vec<Genome>,
    //扫雷机
    sweepers: Vec<MineSweeper>,
    //地雷
    mines: Vec<Vector2D>,
    //GA
    ga: GenAlg,
    num_sweepers: usize,
    num_mines: usize,
    //num_weights_in_nn: i32,
    //扫雷机形状的顶点的顶点缓冲区
    sweeper_vb: Vec<PointF>,
    //顶点缓冲区，用于地雷形状的顶点
    mine_vb: Vec<PointF>,
    //存储每一代的平均适合度以用于绘图。
    av_fitness: Vec<f64>,
    best_fitness: Vec<f64>,
    red_pen: HPEN,
    blue_pen: HPEN,
    green_pen: HPEN,
    old_pen: HPEN,
    //hwnd_main: HWND,
    //切换模拟运行的速度
    fast_render: bool,
    render_enable: bool,
    //每一代的周期
    ticks: i32,
    //世代计数器
    generations: i32,
    //窗口尺寸
    cx_client : i32,
    cy_client: i32,

}

impl Drop for Controller {
    fn drop(&mut self) {
        println!("Drop Controller...");
        //删除HPEN
        unsafe{
            gdi32::DeleteObject(self.blue_pen as HGDIOBJ);
            gdi32::DeleteObject(self.red_pen as HGDIOBJ);
            gdi32::DeleteObject(self.green_pen as HGDIOBJ);
            gdi32::DeleteObject(self.old_pen as HGDIOBJ);
        }
    }
}

impl Controller{
    //创建Controller的实例时，会有一系列事情发生
    // 1.创建Minesweeper对象。
    // 2.统计神经网络中所使用的权重总数，然后此数字被利用来初始化遗传算法类的一个实例。
    // 3.从遗传算法对象中随机提取染色体（权重）并插入到扫雷机的神经网络中。
    // 4.创建大量的地雷并被随机的散播到各地。
    // 5.为绘图函数创建所有的GDI画笔
    // 6.为扫雷机和地雷的形状创建顶点缓冲区

    pub fn new() -> Controller {
        println!("Controller::new");
        let sweeper = vec![PointF::from(-1.0, -1.0),
                PointF::from(-1.0, 1.0),
                PointF::from(-0.5, 1.0),
                PointF::from(-0.5, -1.0),

                PointF::from(0.5, -1.0),
                PointF::from(1.0, -1.0),
                PointF::from(1.0, 1.0),
                PointF::from(0.5, 1.0),
                
                PointF::from(-0.5, -0.5),
                PointF::from(0.5, -0.5),

                PointF::from(-0.5, 0.5),
                PointF::from(-0.25, 0.5),
                PointF::from(-0.25, 1.75),
                PointF::from(0.25, 1.75),
                PointF::from(0.25, 0.5),
                PointF::from(0.5, 0.5)];
        let mine:Vec<PointF> = vec![PointF::from(-1.0, -1.0),
                PointF::from(-1.0, 1.0),
                PointF::from(1.0, 1.0),
                PointF::from(1.0, -1.0)];
        
        //让我们创建扫雷器
        let mut sweepers: Vec<MineSweeper> = vec![];
        let mut mines: Vec<Vector2D> = vec![];
        for _ in 0..NUM_SWEEPERS {
            sweepers.push(MineSweeper::new());
        }
        //获取扫描器NN中使用的权重总数，以便我们可以初始化GA
        let num_weights_in_nn = sweepers[0].get_number_of_weights();
        //pub fn new(popsize: usize, mut_rat: f64, cross_rat: f64, num_weights: i32) -> GenAlg
        let ga =  GenAlg::new(
                NUM_SWEEPERS,
                MUTATION_RATE,
                CROSSOVER_RATE,
                num_weights_in_nn,
                sweepers[0].calculate_split_points());

        //从GA获取权重(初始化时随机的)，并插入扫描器大脑
        let the_population = ga.get_chromos();

        for i in 0..NUM_SWEEPERS {
            sweepers[i].put_weights(&the_population[i].weights);
        }

        //在应用程序窗口内的随机位置初始化地雷
        for _ in 0..NUM_MINES {
            mines.push(Vector2D::new(
                random_float()*WINDOW_WIDTH as f64,
                random_float()*WINDOW_HEIGHT as f64));
        }

        //填充顶点缓冲区
        let mut sweeper_vb: Vec<PointF> = vec![];
        let mut mine_vb: Vec<PointF> = vec![];

        for i in 0..NUM_SWEEPER_VERTS {
            sweeper_vb.push(sweeper[i].clone());
        }
        for i in 0..NUM_MINE_VERTS {
            mine_vb.push(mine[i].clone());
        }

        Controller {
            num_sweepers: NUM_SWEEPERS,
            ga: ga,
            fast_render:false,
            render_enable: true,
            ticks: 0,
            num_mines: NUM_MINES,
            //hwnd_main: hwnd_main,
            generations: 0,
            the_population: the_population,
            sweepers: sweepers,
            mines: mines,
            cx_client: WINDOW_WIDTH,
            cy_client: WINDOW_HEIGHT,
            blue_pen: unsafe{ gdi32::CreatePen(PS_SOLID, 1, rgb(0, 0, 255) as COLORREF) },
            red_pen: unsafe{ gdi32::CreatePen(PS_SOLID, 1, rgb(255, 0, 0) as COLORREF) },
            green_pen: unsafe { gdi32::CreatePen(PS_SOLID, 1, rgb(0, 150, 0) as COLORREF) },
            old_pen: 0 as HPEN,
            sweeper_vb: sweeper_vb,
            mine_vb:mine_vb,
            av_fitness: vec![],
            best_fitness: vec![],
            //num_weights_in_nn: num_weights_in_nn,
        }
    }

    //此函数绘制了运行过程中的平均值和最佳拟合度的图表
    //给定一个在这个函数上绘制的曲面显示统计数据和一个显示最佳和平均适合度的粗略图形
    fn plot_stats(&mut self, surface: HDC) {
        unsafe{
            let s = format!("最好适应性分数: {}", self.ga.best_fitness());
            text_out(surface, 5, 20, &s);

            let s = format!("平均适应性分数: {}", self.ga.average_fitness());
            text_out(surface, 5, 40, &s);

            //绘制图形
            let mut h_slice = self.cx_client / (self.generations+1);
            if h_slice < 1 {
                h_slice = 1;
            }
            let v_slice = self.cy_client as f64 / ((self.ga.best_fitness()+1.0)*2.0);

            //绘制最佳适应分图
            let mut x = 0.0;
            self.old_pen = gdi32::SelectObject(surface, self.red_pen as HGDIOBJ) as HPEN;

            move_to_ex(surface, 0.0, self.cy_client as f64);

            for i in 0..self.best_fitness.len() {
                line_to(surface, x, self.cy_client as f64 - v_slice* self.best_fitness[i]);
                x += h_slice as f64;
            }
            
            //绘制平均适合度的图表
            x = 0.0;
            gdi32::SelectObject(surface, self.blue_pen as HGDIOBJ);
            move_to_ex(surface, 0.0, self.cy_client as f64);

            for i in 0..self.av_fitness.len() {
                line_to(surface, x, self.cy_client as f64 - v_slice* self.av_fitness[i]);
                x += h_slice as f64;
            }
            //恢复PEN
            gdi32::SelectObject(surface, self.old_pen as HGDIOBJ);
        }
    }

    //设置地雷的转换矩阵，并将世界变换应用于传递给此方法的顶点缓冲区中的每个顶点。
    pub fn world_transform(buffer: &mut Vec<PointF>, pos: &Vector2D){
        let mut mat_transfrom = Matrix::new();
        mat_transfrom.scale(MINE_SCALE, MINE_SCALE);
        mat_transfrom.translate(pos.x, pos.y);
        mat_transfrom.transform_points(buffer);
    }

    //在每一帧中被调用来对扫雷机进行演化
    //1.对所有扫雷机进行循环，如发现某一扫雷机找到了地雷，就更新该扫雷机的适应性分数。
    //2.the_population 包含了所有基因组的备份，相关适应性分数也要在这时进行调整。
    //3.如果完成一个代(generation)so需要的帧数均已通过，执行一个遗传算法时代(epoch)来产生新一代的权重。
    //  这些权重被用来替代扫雷机神经网络中原有的旧的权重，是扫雷机的每一个参数被重新设置，
    //  从而为进入新一代做好准备。
    pub fn update(&mut self) -> bool {
        //NumTicks代表了扫雷机运行多久，进行下一代进化。
        //在此循环中，每个扫雷机的神经网络不断利用它周围的环境信息进行更新
        //而从神经网络得到的输出使扫雷机实现所需要的动作
        //如果扫雷机遇见了一个地雷，则它的适应性分数将相应地被更新
        //同样地更新它对应基因组的适应性分数
        //println!("ctrl->update");
        if self.ticks < NUM_TICKS {
            self.ticks += 1;
            //println!("tickes==={:?}", self.ticks);
            for i in 0..self.num_sweepers {
                //根据神经网络的输出更新扫雷机位置
                if !self.sweepers[i].update(&self.mines) {
                    //神经网络处理出错
                    unsafe {
                        user32::MessageBoxW(null_mut(), str_to_ws("NN输入数量错误！").as_ptr(), str_to_ws("错误").as_ptr(), winapi::MB_OK);
                        return false;
                    }
                }
                //看是否找到了一个地雷
                let grab_hit = self.sweepers[i].check_for_mine(&self.mines, MINE_SCALE);
                if grab_hit >0 {
                    //我们发现了一个雷so增加适应分
                    self.sweepers[i].increment_fitness();
                    //地雷被发现，so随机改变一下它的位置
                    self.mines[grab_hit as usize] =Vector2D::new(
                            random_float()*WINDOW_WIDTH as f64,
                            random_float()*WINDOW_HEIGHT as f64);
                }
                //更新扫雷机对应的染色体的适应分(发现的雷的数量)
                self.the_population[i].fitness = self.sweepers[i].fitness();
            }
        }

        //进化到下一代, the_population 中存储了各个扫雷机对应的染色体以及他们的适应性得分
        //是时间运行GA和更新扫雷机与他们的新NNs了
        //以下程序为运行遗传算法并用它们新的神经网络更新扫雷机
        else {
            //最好适应分和平均适应分用于在窗口中展示
            self.av_fitness.push(self.ga.average_fitness());
            self.best_fitness.push(self.ga.best_fitness());
            println!("Generation: {} 最好: {:?} 平均: {:?}", self.generations, self.ga.best_fitness(), self.ga.average_fitness());
            //时代计数器+1
            self.generations += 1;
            //重置循环
            self.ticks = 0;

            //运行GA创建新的群体
            self.the_population = self.ga.epoch(&mut self.the_population);
            //将新一代的染色体(基因组)分别放入到扫雷机的神经网络大脑中
            for i in 0..self.num_sweepers {
                self.sweepers[i].put_weights(&self.the_population[i].weights);
                self.sweepers[i].reset();
            }
        }

        //概括起来，程序为每一时代(epoch)做的工作步骤如下:
        // 1.为所有扫雷机和NUM_TICKS个帧组织循环，调用Update函数并根据情况增加扫雷机适应性分数
        // 2.检索扫雷机的ANN的权重数组
        // 3.用遗传算法演化出一个新的网络权重群体
        // 4.把新的权重插入到扫雷机的神经网络
        // 5.转到第1步进行重复, 直至获得理想的性能。
        true
    }

    pub fn render(&mut self, surface: HDC) {
        if !self.render_enable { return; }
        unsafe {
            //绘制状态
            let s = format!("代: {}", self.generations);
            text_out(surface, 5, 0, &s);

            //如果以加速的速度运行，不呈现
            if !self.fast_render {
                self.old_pen = gdi32::SelectObject(surface, self.green_pen as HGDIOBJ) as HPEN;

                //绘制地雷
                for i in 0..self.num_mines {
                    //抓取地雷形状的顶点
                    let mut mine_vb = self.mine_vb.clone();
                    Controller::world_transform(&mut mine_vb, &self.mines[i]);
                    //画地雷
                    move_to_ex(surface, mine_vb[0].x, mine_vb[0].y);
                    for vert in 1..mine_vb.len() {
                        line_to(surface, mine_vb[vert].x, mine_vb[vert].y);
                    }
                    line_to(surface, mine_vb[0].x, mine_vb[0].y);
                }

                //我们希望fittest显示为红色
                gdi32::SelectObject(surface, self.red_pen as HGDIOBJ);
                //render the sweepers
                for i in 0..NUM_SWEEPERS {
                    if i == NUM_ELITE {
                        gdi32::SelectObject(surface, self.old_pen as HGDIOBJ);
                    }

                    let mut sweeper_vb = self.sweeper_vb.clone();
                    self.sweepers[i].world_transform(&mut sweeper_vb);
                    //画扫雷机的左轮
                    move_to_ex(surface, sweeper_vb[0].x, sweeper_vb[0].y);
                    for vert in 1..4{
                        line_to(surface, sweeper_vb[vert].x, sweeper_vb[vert].y);
                    }
                    line_to(surface, sweeper_vb[0].x, sweeper_vb[0].y);
                    //画扫雷机的右轮
                    move_to_ex(surface, sweeper_vb[4].x, sweeper_vb[4].y);
                    for vert in 5..8{
                        line_to(surface, sweeper_vb[vert].x, sweeper_vb[vert].y);
                    }
                    line_to(surface, sweeper_vb[4].x, sweeper_vb[4].y);

                    move_to_ex(surface, sweeper_vb[8].x, sweeper_vb[8].y);
                    line_to(surface, sweeper_vb[9].x, sweeper_vb[9].y);

                    move_to_ex(surface, sweeper_vb[10].x, sweeper_vb[10].y);
                    for vert in 11..16{
                        line_to(surface, sweeper_vb[vert].x, sweeper_vb[vert].y);
                    }
                }
                //恢复old pen
                gdi32::SelectObject(surface, self.old_pen as HGDIOBJ);
            }//end if
            else {
                self.plot_stats(surface);
            }
        }
    }
    
    pub fn fast_render_toggle(&mut self){
        self.fast_render = !self.fast_render;
    }

    pub fn render_enable_toggle(&mut self){
        self.render_enable = !self.render_enable;
    }

    pub fn fast_render(&self) -> bool{
        self.fast_render
    }
}