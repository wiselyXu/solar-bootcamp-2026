use std::{cell::RefCell, rc::Rc};


/**
 * 练习的是图相关的内容 ， 还有堆的处理， 相对可能比较难
 */
pub fn sub_main(){
   // println!("from leetcode/day5/function.rs")
   test_matrix();
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

use std::collections::VecDeque;
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