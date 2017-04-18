use neural_net::{ NeuralNet };
use vector_2d::{Vector2D};
use utils::{ random_float,clamp, PointF };
use matrix::Matrix;
use params::{ START_ENERGY, WINDOW_HEIGHT, NUM_OUTPUTS, WINDOW_WIDTH, SWEEPER_SCALE,TWO_PI,MAX_TURN_RATE };

pub struct MineSweeper {
    //扫雷机的神经网络
    its_brain: NeuralNet,
    //扫雷机在世界坐标里的位置
    position: Vector2D,
    //扫雷机面对的方向
    look_at: Vector2D,
    //扫雷机的旋转
    rotation: f64,
    speed: f64,
    //以存储来自ANN的输出
    left_track: f64,
    right_track: f64,
    //用于度量扫雷机适应性分数
    fitness: f64,
    scale: f64,
    //最近的地雷的位置
    closest_mine: usize,
}

impl MineSweeper {
    pub fn new() ->MineSweeper {
        MineSweeper {
            rotation: random_float() * TWO_PI,
            left_track: 0.16,
            right_track: 0.16,
            fitness: START_ENERGY,
            scale: SWEEPER_SCALE as f64,
            closest_mine: 0,
            position: Vector2D::new(random_float()*WINDOW_WIDTH as f64, random_float()*WINDOW_HEIGHT as f64),
            speed: 0.0,
            look_at: Vector2D::new(0.0, 0.0),
            its_brain: NeuralNet::new(),
        }
    }

    pub fn get_number_of_weights(&self)->i32 {
        self.its_brain.get_number_of_weights()
    }

    pub fn put_weights(&mut self, w: &Vec<f64>){
        self.its_brain.put_weights(w)
    }

    pub fn fitness(&self) -> f64 {
        self.fitness
    }

    pub fn increment_fitness(&mut self){
        self.fitness += 1.0;
    }

    pub fn decrease_fitness(&mut self){
        self.fitness -= 1.0;
    }

    pub fn position(&self) ->&Vector2D {
        &self.position
    }

    //重置扫地机的位置，适应分和旋转
    pub fn reset(&mut self) {
        self.position = Vector2D::new(random_float()*WINDOW_WIDTH as f64, random_float()*WINDOW_HEIGHT as f64);
        self.fitness = START_ENERGY;
        self.rotation = random_float() * TWO_PI;
    }

    //用来对扫雷机各个顶点进行变换，以便下一步可以画出扫雷机
    pub fn world_transform(&self, sweeper: &mut Vec<PointF>) {
        let mut mat_transfrom = Matrix::new();
        mat_transfrom.scale(self.scale, self.scale);
        mat_transfrom.rotate(self.rotation);
        mat_transfrom.translate(self.position.x, self.position.y);
        mat_transfrom.transform_points(sweeper);
    }

    //利用扫雷机环境得到的信息来更新人工神经网络
    //首先，我们采取传感器读数，并将这些进入扫地机的大脑。
    //
    //输入是：
    //
    // 到最近的我的向量(x, y)
    // 扫雷机的朝向向量(x, y)
    //
    //我们从大脑接收两个输出。lTrack&rTrack。
    //因此给定每个轨道的力，我们计算结果的旋转和加速度，并应用于当前速度矢量。
    pub fn update(&mut self, mines: &Vec<Vector2D>) -> bool{
        //这一向量用来存放神经网络所有的输入
        let mut inputs:Vec<f64> = vec![];
        //计算从扫雷机到与其最近的地雷（两个点）之间的向量
        let mut closest_mine = self.get_closest_mine(mines);
        //将该向量规范化(扫雷机的视线向量不需要再做规范化，因为它的长度已经等于1了)
        //println!("1==={:?}", closest_mine);
        Vector2D::normalize(&mut closest_mine);

        //计算向量和最近的矿井矢量的点积。 这将给我们的角度，我们需要转向面对最近的矿井
        let dot = Vector2D::dot(&self.look_at, &closest_mine);
        let sign = Vector2D::sign(&self.look_at, &closest_mine);

        inputs.push(dot*sign as f64);

        // //加入扫雷机到最近地雷之间的向量
        // inputs.push(closest_mine.x);
        // inputs.push(closest_mine.y);
        // //加入扫雷机的视线向量
        // inputs.push(self.look_at.x);
        // inputs.push(self.look_at.y);
        //更新大脑，并从网络得到输出
        let output = self.its_brain.update(&mut inputs);
        

        //确保在计算输出时没有错误
        if output.len() < NUM_OUTPUTS as usize {
            return false;
        }
        //把输出复制到扫雷机的左、右履带轮轨
        self.left_track = output[0];
        self.right_track = output[1];

        //计算驾驶的力
        //扫雷机的转动力是利用施加到它左、右履带轮轨上的力之差来计算的。
        //并规定，施加到左轨道上的力减去右轨道上的力，就得到扫雷机车辆的转动力。
        let mut rot_force = self.left_track - self.right_track;

        //进行左转或右转
        clamp(&mut rot_force, -MAX_TURN_RATE, MAX_TURN_RATE);
        self.rotation += rot_force;
        //扫雷机车的行进速度为它的左侧轮轨速度与它的右侧轮轨速度的和。
        self.speed = self.left_track + self.right_track;

        //更新视线角度
        self.look_at.x = -(self.rotation.sin());
        self.look_at.y = self.rotation.cos();

        //更新位置
        self.position += Vector2D::mul(&self.look_at, self.speed);
        
        //屏幕越界处理
        if self.position.x > WINDOW_WIDTH as f64 { self.position.x = 0.0; }
        if self.position.x < 0.0 { self.position.x = WINDOW_WIDTH as f64; }
        if self.position.y > WINDOW_HEIGHT as f64 { self.position.y = 0.0; }
        if self.position.y < 0.0 { self.position.y = WINDOW_HEIGHT as f64; }

        true
    }

    //检查扫雷机看它是否已经发现地雷
    //此函数检查与其最近的矿区的碰撞（先计算并存储在self.closest_mine中）
    pub fn check_for_mine(&self, mines: &Vec<Vector2D>, size: f64) -> i32 {
        let dist_to_object = Vector2D::sub(&self.position, &mines[self.closest_mine]);
        //println!("dist_to_object.len() = {}", dist_to_object.len());
        if Vector2D::length(&dist_to_object) < (size+5.0) {
            return self.closest_mine as i32;
        }
        -1
    }

    //返回一个向量到最邻近的地雷
    pub fn get_closest_mine(&mut self, mines: &Vec<Vector2D>) ->Vector2D {
        let mut closest_so_far = 99999.0;
        let mut closest_object = Vector2D::new(0.0, 0.0);
        for i in 0..mines.len() {
            let len_to_object = Vector2D::length(&Vector2D::sub(&mines[i], &self.position));
            if len_to_object < closest_so_far {
                closest_so_far = len_to_object;
                closest_object = Vector2D::sub(&self.position, &mines[i]);
                self.closest_mine = i;
            }
        }
        closest_object
    }

    pub fn calculate_split_points(&self) ->Vec<i32> {
        self.its_brain.calculate_split_points()
    }
}