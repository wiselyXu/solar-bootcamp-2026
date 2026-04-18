use std::{cell::RefCell, rc::Rc};


/**
 * 练习的是图相关的内容 ， 还有堆的处理， 相对可能比较难
 */
pub fn sub_main(){
   // println!("from leetcode/day5/function.rs")
 //  test_matrix();
 //test_207()
 test_vec();
}


fn test_vec(){

    let mut graph  = vec![vec![];3];
    graph[0] = vec![1,3,4];
    graph[1] = vec![7,8];
    graph[2] = vec![9];

   // let a = graph[2][1];   它仍然 表示是 i32 ， 尽管  不存在这个下标了， 所以这个编译器， 没法检测。
  //  println!("graph[2][1] = {}", graph[2][1] );
    println!("graph[1][1] = {}", graph[1][1]);



}

/**
 * leecode 200： 岛的数量计算， 这个中等难度 ， java 实现过
 * 每个 坐标， 要么 1 ， 要么 0  ， 
 * 思路很清晰， 遍历每一个元素， 按如下规则
 * 如果当前元素不是 0 
 * 1）往前后左右四个方面 尝试， 即递归
 * 2）当前无素变为 0
 * 
 * 这 个写法， 能通过 测试， 但时间复杂度， 为最后， 23ms
 */
fn num_islands_200(grid: Vec<Vec<char>>) -> i32 {
    //let matrix = Rc::new(RefCell::new(grid));
    let lines = grid.len();
    let columns = grid[0].len();

    let mut grid = grid; // 转为可变矩阵

    let mut islands = 0;
    for i in 0..lines {
        for j in 0.. columns {
            if grid[i][j] == '1' {
                islands +=1;
                grid[i][j] = '0';

                // 尝试将4 个方向转为 ‘0’
                grid = dye_0(grid.clone(), i,j,lines , columns);  // usize 实现了copy trait， 就不会传所有权吗？ 不需要再 clone()了
            }
        }
    }

   islands   
}


fn dye_0( mut grid:  Vec<Vec<char>>,  i:usize,j:usize, lines: usize, columns: usize) -> Vec<Vec<char>>{
     
     // 下  i +1
     let mut m = (i+1) ;
     let mut n = j ;
     if m <lines  && grid[m][n] == '1' {
        grid[m as usize][n] = '0';
        grid = dye_0(grid, m,n,lines,columns);
     }

     // 上  i -1
     m = i-1;
     n =j ;
     if m as i32 >= 0 && grid[m][n] == '1' {
        grid[m][n] = '0';
        grid = dye_0(grid, m,n,lines,columns);
     }
     
     // 左  j -1
     m = i;
     n =j -1 ;
     if n as i32 >= 0 && grid[m][n] == '1' {
        grid[m][n] = '0';
        grid = dye_0(grid, m,n,lines,columns);
     }

     // 右 j +1
     
     m = i;
     n =j +1 ;
     if n  < columns && grid[m][n] == '1' {
        grid[m][n] = '0';
        grid = dye_0(grid, m,n,lines,columns);
     }
    grid
}

use std::collections::{HashMap, HashSet, VecDeque};
/**
 * 看注释， 是一越南人的
 * 它的关键就是将 需要 扩展访问的节点  放到queue中， 直到queue 取完， 这一个岛， 就算完成了
 *   这样就不用递归 ，而改为循环处理即可 ，  
 * 第二点的设计是  向4个方向移动 ，使用 [(1,0),(-1,0),(0,1),(0,-1)], 然后加完 ，就按同一逻辑来处理，这里没有任何借用问题
 * 我用v3 版本， 自己再写一下
 * 
 */
fn num_islands_200_v2(mut grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut islands_count = 0;

    for r in 0..m {
        for c in 0..n {
            if grid[r][c] == '1' {
                islands_count += 1; 
                
                // Khởi tạo Queue theo đúng ý tưởng của bro
                let mut queue = VecDeque::new();
                queue.push_back((r, c));
                
                // ĐÁNH CHÌM ngay lập tức để đánh dấu là "Visited"
                grid[r][c] = '0';

                // Bắt đầu vòng lặp lan rộng (BFS)
                while let Some((curr_r, curr_c)) = queue.pop_front() {
                    // 4 hướng: Lên, Xuống, Trái, Phải
                    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    
                    for (dr, dc) in dirs.iter() {
                        let next_r = curr_r as i32 + dr;
                        let next_c = curr_c as i32 + dc;

                        // Kiểm tra xem tọa độ hàng xóm có hợp lệ không (không vượt rào) 
                        // và quan trọng nhất: nó có phải là đất ('1') không?
                        if next_r >= 0 && next_r < m as i32 && 
                           next_c >= 0 && next_c < n as i32 && 
                           grid[next_r as usize][next_c as usize] == '1' 
                        {
                            // Ném hàng xóm vào Queue để chờ khám phá
                            queue.push_back((next_r as usize, next_c as usize));
                            // Lại ĐÁNH CHÌM nó (Visited)
                            grid[next_r as usize][next_c as usize] = '0';
                        }
                    }
                }
                // Bất cứ khi nào Queue rỗng -> Kết thúc trọn vẹn 1 hòn đảo (đã chìm ráo trọi)
            }
        }
    }

    islands_count

}


/**
 * 不太用  queue.is_empty() ， 而直接  while let((cr,cc)) = queue.pop_front()
 */
fn num_islands_200_v3(grid: Vec<Vec<char>>) -> i32 {

    let mut grid = grid;
    let mut islands = 0;
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let columns = grid[0].len();
    if columns == 0 {
        return 0;
    }

    let directions = [(1,0),(-1,0),(0,1),(0,-1)];
    
    for r in 0..rows {
        for c in 0..columns {
            if grid[r][c] == '1' {
                islands +=1;
                grid[r][c] = '0';

                let mut queue = VecDeque::new();
                queue.push_back((r,c));
                while let Some((cr,cc)) = queue.pop_front() {
                    for (ar,ac) in directions {
                        let nr = cr as i32 + ar ;
                        let nc = cc as i32 + ac ;
                        if nr >=0 && nr < rows as i32 &&
                           nc >=0 && nc < columns as i32  &&
                           grid[nr as usize][nc as usize] == '1' {
                            grid[nr as usize][nc as usize] = '0';

                            queue.push_back((nr as usize, nc as usize));
                           }
                    }
                }
                
            }
        }
    }

    islands
}

fn test_matrix() -> Vec<Vec<i32>>{
    let mut matrix = Vec::new();
    
    let line1 = vec![1,2,3,4,5];
    let line2 = vec![10,20,30,40,50];
    let line3 = vec![100,200,300,400,500];
    matrix.push(line1);
    matrix.push(line2.clone());
    matrix.push(line3);
    matrix.push(line2);

    println!("m(2,3) 应该是 400， 实际  {}", matrix[2][3]);
    matrix
}


/**
 * 这个作为了一个中级的题目
 * 看课程安排是否 可安排， 最重要的就是是否有依赖环的出现， 如果有，就安排不了
 * 每个依赖描述， 就2元素  [a,b] ， 完成 a,的前提，要完成b ， 那就要看b 是否间接依赖a了 ，如 [b,c]  [c,d] [d,a]， 这样就不行
 * a,有环，， 但了,b, c,d 都没环， 也是不行的。 
 * b， 所有的依赖， 如果  b 中依赖的内容， 有a ， 则  a 也是没用
 * 所以定义 一个 hashMap  
 * 直接用 prequisites 来处理即可
 * 梳理出所以，
 * 
 * 
 * 我这么做， 超时。   40/54 的案例通过
 */
fn can_finish_207(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut course_dep_map = HashMap::new();

    // 第一遍循环， 几乎不会有重复的情况  
    for pair in prerequisites {
        course_dep_map.entry(pair[0]).or_insert(HashSet::new()).insert(pair[1]);
        let set = course_dep_map.get(&pair[0]);
        if set.as_ref().is_some_and(|s| s.contains(&pair[0])) {
            return false;
        }
    }

    // 第二遍是对  map 循环， 以获得最全量的依赖, 有一个递归操作， 递归改循环，也好。 如果出现 循环依赖， 必然不能出来， 也能判断出是否包含自己， 
    
    for (key,mut set) in course_dep_map.clone() {
        let mut queue = VecDeque::new();
      //  set.iter().map(|&a| queue.push_back(a));   // map 是惰性的， 要调它才行
          set.iter().for_each(|&a| queue.push_back(a));  // 这个意思更好
          // queue.extend(set.iter()); 这个最简洁
        
        while !queue.is_empty() {
            let course_id = queue.pop_front().unwrap();
            let temp_set = course_dep_map.get(&course_id);
            if temp_set.is_none() {
                continue;
            }
            let temp_set = temp_set.unwrap();
            if temp_set.is_subset(&set) {
                    continue;
            }

            let diff: HashSet<i32> = temp_set.difference(&set).copied().collect();
            if diff.contains(&key) {
                return false;
            }else{
                queue.extend(diff.iter());
            }   
        }
        
    }

    true
        
}


/*
看 AI 给写的， 就是用邻接矩阵， 来解决， ， 最终能安排 num_courses 个课程 ，那就表示，可以安排，
  如果课程 ，完全没有 入度， 说明， 不需要 先修课程 ， 直接放已安排的就好， 然后各个有依赖的， 它的入度， 表示依赖几个， 邻接图， 存了依赖哪些

  写完这一题， 其实就是要记结论， 因为我感觉 有些数学原理 ， 不一定想的起。 
  入度一度为 0 的, 不会断层。
 */
fn can_finish_207_v2(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut arranged = 0;
    let n = num_courses as usize;

    // 构建邻接矩阵  和 入度表
    let mut graph = vec![vec![]; n];
    let mut in_degree = vec![0;n];
    for pair in prerequisites {
        let a = pair[0] as usize;
        let b = pair[1] as usize;
        // 学习a 的前提 是已学习了b， 即b 是a 的先修改课  ， 在图中  b  -> a  ， 入度， a的入度就增加 1
        graph[b].push(a);
        in_degree[a] +=1;

    }

    // 将 不需要 先置课程的 （即入度为0） 放入 队列中，  后面只是入度 为0 了 都放 queue中
    let mut queue = VecDeque::new();   
    for i in 0..n {
        if in_degree[i] == 0 {
            queue.push_back(i);
        }
    }
    // 其实是否存在， 能安排 好， 但又起初没有 入度为 0 的？ 这是个数学 证明题呀， 短时间证明不好。

   while let Some(bef) =   queue.pop_front() {
      arranged +=1;
      // 对它所有的 指向的图， 进行入度减少1 
      let neighbors = std::mem::take(&mut graph[bef]);  // 这样， 所有的neighbor 都拿掉了。 本身也应该这样, 这样 入度减少， 那图上的 邻居也要减掉， 才一致。 虽然对解题 没帮助
      for after in neighbors {
    //for &after in &graph[bef] {
        in_degree[after] -=1;
        if in_degree[after] ==0 {
            queue.push_back(after);
        }

      }
   }
    arranged == num_courses

}


fn test_207(){
   // let prerequistes = vec![vec![1,0],vec![0,1]];
   let prerequistes = vec![vec![1,0]];
    println!("{}",can_finish_207(2,prerequistes));
}