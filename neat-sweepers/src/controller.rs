use params::{ NUM_TICKS, MINE_SCALE, WINDOW_WIDTH, WINDOW_HEIGHT, NUM_MINES, NUM_SWEEPERS };
use std::ptr::null_mut;

use neat::ga::GA;

//控制器
use mine_sweeper::{ MineSweeper };
use vector_2d::{Vector2D};
use matrix::Matrix;
use utils::{ random_float, PointF, rgb };
use win::ui;
use params;

const NUM_SWEEPER_VERTS :usize = 16;
const NUM_MINE_VERTS :usize = 4;

pub struct Controller {
    //扫雷机
    sweepers: Vec<MineSweeper>,
    //地雷
    mines: Vec<Vector2D>,
    //GA
    ga: GA,
    num_sweepers: usize,
    num_mines: usize,
    //扫雷机形状的顶点的顶点缓冲区
    sweeper_vb: Vec<PointF>,
    //顶点缓冲区，用于地雷形状的顶点
    mine_vb: Vec<PointF>,
    //存储每一代的平均适合度以用于绘图。
    av_fitness: Vec<f64>,
    best_fitness: Vec<f64>,
    red_pen: ui::Pen,
    blue_pen: ui::Pen,
    green_pen: ui::Pen,
    old_pen: ui::Pen,
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
        ui::delete_pen(self.blue_pen);
        ui::delete_pen(self.red_pen);
        ui::delete_pen(self.green_pen);
        ui::delete_pen(self.old_pen);
    }
}

impl Controller{
    //创建Controller的实例时，会有一系列事情发生
    // 1.创建Minesweeper对象。
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

        let mut ga = GA::new(NUM_SWEEPERS as i32, params::NUM_INPUTS, params::NUM_OUTPUTS);
        ga.create_phenotypes();
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
            generations: 0,
            sweepers: sweepers,
            mines: mines,
            cx_client: WINDOW_WIDTH,
            cy_client: WINDOW_HEIGHT,
            blue_pen: ui::solid_pen(1, rgb(0, 0, 255)),
            red_pen: ui::solid_pen(1, rgb(255, 0, 0)),
            green_pen: ui::solid_pen(1, rgb(0, 150, 0)),
            old_pen: 0 as ui::Pen,
            sweeper_vb: sweeper_vb,
            mine_vb:mine_vb,
            av_fitness: vec![],
            best_fitness: vec![],
        }
    }

    //此函数绘制了运行过程中的平均值和最佳拟合度的图表
    //给定一个在这个函数上绘制的曲面显示统计数据和一个显示最佳和平均适合度的粗略图形
    fn plot_stats(&mut self, surface: ui::Surface) {
        let s = format!("最好适应性分数: {}", self.ga.best_ever_fitness());
        ui::text_out(surface, 5, 20, &s);

        let s = format!("平均适应性分数: {}", self.ga.average_fitness());
        ui::text_out(surface, 5, 40, &s);

        //绘制图形
        let mut h_slice = self.cx_client / (self.generations+1);
        if h_slice < 1 {
            h_slice = 1;
        }
        let v_slice = self.cy_client as f64 / ((self.ga.best_ever_fitness()+1.0)*2.0);

        //绘制最佳适应分图
        let mut x = 0.0;
        self.old_pen = ui::select_pen(surface, self.red_pen);

        ui::move_to_ex(surface, 0.0, self.cy_client as f64);

        for i in 0..self.best_fitness.len() {
            ui::line_to(surface, x, self.cy_client as f64 - v_slice* self.best_fitness[i]);
            x += h_slice as f64;
        }
        
        //绘制平均适合度的图表
        x = 0.0;
        ui::select_pen(surface, self.blue_pen);
        ui::move_to_ex(surface, 0.0, self.cy_client as f64);

        for i in 0..self.av_fitness.len() {
            ui::line_to(surface, x, self.cy_client as f64 - v_slice* self.av_fitness[i]);
            x += h_slice as f64;
        }
        //恢复PEN
        ui::select_pen(surface, self.old_pen);
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
    //2.如果完成一个代(generation)so需要的帧数均已通过，执行一个遗传算法时代(epoch)来产生新一代的权重。
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
                if !self.sweepers[i].update(&self.mines, i, &mut self.ga) {
                    //神经网络处理出错
                    ui::message_box("错误", "NN输入数量错误！");
                    return false;
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
            }
            
        }

        //进化到下一代
        //是时间运行GA和更新扫雷机与他们的新NNs了
        //以下程序为运行遗传算法并用它们新的神经网络更新扫雷机
        else {
            //最好适应分和平均适应分用于在窗口中展示
            self.av_fitness.push(self.ga.average_fitness());
            self.best_fitness.push(self.ga.best_ever_fitness());
            println!("Generation: {} 最好: {:?} 平均: {:?}", self.generations, self.ga.best_ever_fitness(), self.ga.average_fitness());
            //时代计数器+1
            self.generations += 1;
            //重置循环
            self.ticks = 0;
            for i in 0..self.num_sweepers {
                self.ga.fitness_scores().push(self.sweepers[i].fitness());
            }
            //运行GA创建新的群体
            self.ga.epoch();
            self.ga.fitness_scores().clear();

            for i in 0..self.num_sweepers {
                self.sweepers[i].reset();
            }
        }
        true
    }

    pub fn render(&mut self, surface: ui::Surface) {
        if !self.render_enable { return; }
        //绘制状态
        let s = format!("代: {}", self.generations);
        ui::text_out(surface, 5, 0, &s);

        self.old_pen = ui::select_pen(surface, self.green_pen);
        ui::rectangle(surface, 0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, ui::rgb(255, 255, 255));
        ui::select_pen(surface, self.old_pen);

        //如果以加速的速度运行，不呈现
        if !self.fast_render {
            self.old_pen = ui::select_pen(surface, self.green_pen);

            //绘制地雷
            for i in 0..self.num_mines {
                //抓取地雷形状的顶点
                let mut mine_vb = self.mine_vb.clone();
                Controller::world_transform(&mut mine_vb, &self.mines[i]);
                //画地雷
                ui::move_to_ex(surface, mine_vb[0].x, mine_vb[0].y);
                for vert in 1..mine_vb.len() {
                    ui::line_to(surface, mine_vb[vert].x, mine_vb[vert].y);
                }
                ui::line_to(surface, mine_vb[0].x, mine_vb[0].y);
            }

            //我们希望fittest显示为红色
            ui::select_pen(surface, self.red_pen);
            //render the sweepers
            for i in 0..NUM_SWEEPERS {
                //绘制最好的2个为红色
                if i == 2 {
                    ui::select_pen(surface, self.old_pen);
                }

                let mut sweeper_vb = self.sweeper_vb.clone();
                self.sweepers[i].world_transform(&mut sweeper_vb);
                //画扫雷机的左轮
                ui::move_to_ex(surface, sweeper_vb[0].x, sweeper_vb[0].y);
                for vert in 1..4{
                    ui::line_to(surface, sweeper_vb[vert].x, sweeper_vb[vert].y);
                }
                ui::line_to(surface, sweeper_vb[0].x, sweeper_vb[0].y);
                //画扫雷机的右轮
                ui::move_to_ex(surface, sweeper_vb[4].x, sweeper_vb[4].y);
                for vert in 5..8{
                    ui::line_to(surface, sweeper_vb[vert].x, sweeper_vb[vert].y);
                }
                ui::line_to(surface, sweeper_vb[4].x, sweeper_vb[4].y);

                ui::move_to_ex(surface, sweeper_vb[8].x, sweeper_vb[8].y);
                ui::line_to(surface, sweeper_vb[9].x, sweeper_vb[9].y);

                ui::move_to_ex(surface, sweeper_vb[10].x, sweeper_vb[10].y);
                for vert in 11..16{
                    ui::line_to(surface, sweeper_vb[vert].x, sweeper_vb[vert].y);
                }
            }
            //恢复old pen
            ui::select_pen(surface, self.old_pen);
        }//end if
        else {
            self.plot_stats(surface);
        }
        
        //绘制NEAT信息
        self.old_pen = ui::select_pen(surface, self.green_pen);
        ui::rectangle(surface, WINDOW_HEIGHT-1, 0, WINDOW_WIDTH*2, WINDOW_HEIGHT, ui::rgb(255, 255, 255));
        ui::select_pen(surface, self.old_pen);

        //绘制最好的4个网络
        let brains:Vec<usize> = self.ga.get_best_phenotypes_from_last_generation();
        if brains.len() > 0{
            let cx_info = WINDOW_WIDTH;
            let cy_info = WINDOW_HEIGHT;
            let sp = 20;
            self.ga.get_phenotype(brains[0]).draw_net(surface, cx_info, sp, cx_info+cx_info/2, cy_info/2-sp*2);
            self.ga.get_phenotype(brains[1]).draw_net(surface, cx_info+cx_info/2, sp, cx_info+cx_info, cy_info/2-sp*2);
            self.ga.get_phenotype(brains[2]).draw_net(surface, cx_info, cy_info/2, cx_info+cx_info/2, cy_info-sp*3);
            self.ga.get_phenotype(brains[3]).draw_net(surface, cx_info+cx_info/2, cy_info/2, cx_info+cx_info, cy_info-sp*3);

            if self.fast_render{
                self.ga.render_species_info(surface, 0, 140, cx_info, 160);
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