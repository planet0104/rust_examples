//遗传算法类。这是基于操作实数Vector元素。 用于调整前馈神经网络中的权重。

use std::cmp::Ordering;
use utils::{ random_clamped, random_float, random_int };
use params::{ NUM_COPIES_ELITE,MAX_PERTURBATION, NUM_ELITE };

//保持每个基因组的结构
#[derive(Debug)]
pub struct Genome{
    pub weights: Vec<f64>,
    pub fitness: f64,
}

impl Genome {
    fn new(w: Option<Vec<f64>>, f: Option<f64>) -> Genome {
        Genome{
            weights: match w {
                    Some(weights) => weights,
                    _ => vec![]
                },
            fitness: match f{
                Some(fitness) => fitness,
                _ => 0.0
            }
        }
    }
}

impl Clone for Genome {
    fn clone(&self) -> Genome {
        Genome{ weights: self.weights.clone(), fitness: self.fitness }
    }

    fn clone_from(&mut self, source: &Self) {
        self.weights = source.weights.clone();
        self.fitness = source.fitness;
    }
}

impl Ord for Genome {
    fn cmp(&self, other: &Genome) -> Ordering {
        self.fitness.partial_cmp(&other.fitness).unwrap()
    }
}

//比较运算符重载
impl PartialOrd for Genome {
    fn partial_cmp(&self, other: &Genome) -> Option<Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}
//等于运算符重载
impl PartialEq for Genome {
    fn eq(&self, other: &Genome) -> bool {
        self.fitness == other.fitness
    }
}
impl Eq for Genome {}

//遗传算法类
pub struct GenAlg{
    //这包含染色体的整个群体
    pop: Vec<Genome>,

    //人口规模
    pop_size: usize,

    //每个染色体的权重量
    chromo_length: i32,

    //总体适应分数
    total_fitness: f64,

    //人口中最高适应分
    best_fitness: f64,

    //平均适应分
    average_fitness: f64,

    //最坏的
    worst_fitness: f64,

    //跟踪最好的基因组
    fittest_genome: usize,

    //染色体位将发生变异的概率。
    //尝试数字在0.05到0.3之间
    mutation_rate: f64,

    //染色体位交叉的概率
    //0.7是相当不错
    crossover_rate: f64,

    //世代计数器
    //generation: i32,

    split_points: Vec<i32>,
}

impl GenAlg{

    //通过将其权重扰动不大于MAX_PERTURBATION的量来突变染色体
    fn mutate(&self, chromo: &mut Vec<f64>){
        //遍历染色体并根据突变率突变每个重量
        for i in 0..chromo.len() {
            //我们扰乱这个重量吗？
            if random_float() < self.mutation_rate {
                //向权重添加或减去一个小值
                chromo[i] += random_clamped() * MAX_PERTURBATION;
            }
        }
    }

    //返回基于轮盘轮采样的基因
    fn get_chromo_roulette(&self) -> &Genome {
        //生成0和总体适应分之间的随机数
        let slice = random_float() * self.total_fitness;
        //这将被设置为选择的染色体
        let mut the_choose_one_idx = 0 as usize;
        //适应分累加
        let mut fitness_so_far = 0.0;
        for i in 0..self.pop_size {
            fitness_so_far += self.pop[i].fitness;
            //如果当前适应分>随机数，返回此处的染色体
            if fitness_so_far >= slice {
                the_choose_one_idx = i;
                break;
            }
        }
        &self.pop[the_choose_one_idx]
    }

    //给定父代和后代的存储，此方法根据GAs交叉率执行交叉
    fn crossover(&self, mum: &Vec<f64>, dad: &Vec<f64>, baby1: &mut Vec<f64>, baby2: &mut Vec<f64>){
        //刚刚返回父母作为后代取决于速率或如果父母是相同的
        if (random_float()>self.crossover_rate) || (mum == dad) {
            *baby1 = mum.clone();
            *baby2 = dad.clone();
            return;
        }
        //确定交叉点
        let cp = random_int(0, self.chromo_length -1) as usize;
        //创建后代，两个后代分别拥有mum和dad的部分基因
        for i in 0..cp {
            baby1.push(mum[i]);
            baby2.push(dad[i]);
        }
        for i in cp..mum.len() {
            baby1.push(dad[i]);
            baby2.push(mum[i]);
        }
    }

    fn crossover_at_splits(&self, mum: &Vec<f64>, dad: &Vec<f64>, baby1: &mut Vec<f64>, baby2: &mut Vec<f64>) {
        //刚刚返回父母作为后代取决于速率或如果父母是相同的
        if (random_float()>self.crossover_rate) || (mum == dad) {
            *baby1 = mum.clone();
            *baby2 = dad.clone();
            return;
        }
        //确定交叉点
        let index1 = random_int(0, self.split_points.len() as i32-2);
        let index2 = random_int(index1, self.split_points.len() as i32-1);

        let cp1 = self.split_points[index1 as usize];
        let cp2 = self.split_points[index2 as usize];

        for i in 0..mum.len() {
            //如果在交叉点以外，保留相同的基因
            if (i as i32)<cp1 || i as i32>=cp2 {
                baby1.push(mum[i]);
                baby2.push(dad[i]);
            }else{
                //切换肚皮块
                baby1.push(mum[i]);
                baby2.push(mum[i]);
            }
        }
        //println!("mum={:?} dad={:?}", mum, dad);
        //println!("baby1={:?} baby2={:?}", baby1, baby2);
    }

    //获取一批染色体，并在循环中运行算法。 返回一个新的染色体群体。
    pub fn epoch(&mut self, old_pop: &mut Vec<Genome>) -> Vec<Genome> {
        //将给定的群体分配给类群体
        self.pop = old_pop.clone();
        //重置相应的变量
        self.reset();
        //排序人口（缩放和精英主义）-- 从小到大排序
        //self.pop.sort();
        self.pop.sort_by(|a, b| b.cmp(a));
        //计算最佳，最差，平均和总体适应度
        self.calculate_best_worst_av_tot();
        //创建一个临时Vec来存储新的染色体
        let mut new_pop: Vec<Genome> = vec![];
        //现在添加一点精英主义，我们将添加一些适合的基因组拷贝。 
        //确保我们添加一个偶数号码或轮盘赌的采样会崩溃
        if NUM_COPIES_ELITE * NUM_ELITE % 2 == 0 {
            self.grab_best(NUM_ELITE, NUM_COPIES_ELITE, &mut new_pop);
        }
        //现在我们进入GA循环
        //重复，直到生成新的群体
        while new_pop.len() < self.pop_size {
            //抓两条染色体
            let mum = self.get_chromo_roulette();
            let dad = self.get_chromo_roulette();

            ////通过crossover创建一些后代
            let mut baby1:Vec<f64> = vec![];
            let mut baby2:Vec<f64> = vec![];
            //self.crossover(&mum.weights, &dad.weights, &mut baby1, &mut baby2);
            self.crossover_at_splits(&mum.weights, &dad.weights, &mut baby1, &mut baby2);
            //进行变异
            self.mutate(&mut baby1);
            self.mutate(&mut baby2);
            //将两个子代复制到人口队列
            new_pop.push(Genome::new(Some(baby1), Some(0.0)));
            new_pop.push(Genome::new(Some(baby2), Some(0.0)));
        }
        //替换到新的群体
        self.pop = new_pop.clone();
        new_pop
    }

    //这种类型的适应分变比将人群按照适应分的升序排序，
    //然后根据其在梯子中的位置简单地分配适应性得分。
    //（所以如果一个基因组最后是最终得分为零，如果最好然后它得到一个等于人口的大小的分数。
    //你还可以分配一个乘数，将增加梯子上基因组的“分离”，并允许 人口收敛得快得多
    // fn fitness_scale_rank(&mut self) {
    //     let fitness_multiplier = 1.0;
    //     //根据基因组在这个新的适应分“梯子”上的位置分配适应度
    //     for i in 0..self.pop_size {
    //         self.pop[i].fitness = i as f64 * fitness_multiplier;
    //     }
    //     //重新计算在选择中使用的值
    //     self.calculate_best_worst_av_tot();
    // }

    ////这类似于精英主义的高级形式，通过将最佳最适合基因组的NumCopies副本插入到总体向量中
    fn grab_best(&self, best: usize, num_copies: usize, pop: &mut Vec<Genome>){
        //将所需的n个最适合的拷贝添加到所提供的向量
        for i in 0..best {
            for _ in 0..num_copies {
                pop.push(self.pop[i].clone());
            }
        }
    }

    //计算适合度和最弱的基因组和平均/总体适应度分数
    fn calculate_best_worst_av_tot(&mut self) {
        self.total_fitness = 0.0;
        let mut highest_so_far = 0.0;
        let mut lowest_so_far = 9999999.0;
        for i in 0..self.pop_size {
            //如有必要，更新fittest
            if self.pop[i].fitness > highest_so_far {
                highest_so_far = self.pop[i].fitness;
                self.fittest_genome = i;
                self.best_fitness = highest_so_far;
            }
            //如有必要，更新最差
            if self.pop[i].fitness < lowest_so_far {
                lowest_so_far = self.pop[i].fitness;
                self.worst_fitness = lowest_so_far;
            }
            self.total_fitness += self.pop[i].fitness;
        }//下一个染色体
        self.average_fitness = self.total_fitness / self.pop_size as f64;
    }

    fn reset(&mut self){
        self.total_fitness = 0.0;
        self.best_fitness = 0.0;
        self.worst_fitness = 9999999.0;
        self.average_fitness = 0.0;
    }

    pub fn new(popsize: usize, mut_rat: f64, cross_rat: f64, num_weights: i32, split_points:Vec<i32>) -> GenAlg{
        let mut pop = vec![];
        //初始化人口，染色体由随机权重组成，所有适应度设置为零
        for i in 0..popsize {
            pop.push(Genome::new(None, None));
            for _ in 0..num_weights {
                pop[i].weights.push(random_clamped());
            }
        }

        GenAlg{
            pop_size: popsize,
            mutation_rate: mut_rat,
            crossover_rate: cross_rat,
            chromo_length: num_weights,
            total_fitness: 0.0,
            //generation: 0,
            fittest_genome: 0,
            best_fitness: 0.0,
            worst_fitness: 99999999.0,
            average_fitness: 0.0,
            pop: pop,
            split_points: split_points,
        }
    }

    pub fn get_chromos(&self) -> Vec<Genome> {
        self.pop.clone()
    }

    pub fn average_fitness(&self) -> f64{ self.total_fitness / self.pop_size as f64 }
    pub fn best_fitness(&self) -> f64 { self.best_fitness }
}