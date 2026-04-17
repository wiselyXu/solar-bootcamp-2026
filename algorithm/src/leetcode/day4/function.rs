use std::{cell::{Ref, RefCell}, collections::{LinkedList, VecDeque}, f32::consts::LOG2_10, rc::Rc};


/** 
 * 本文件 是  树相关的算法 
 */
pub fn sub_main(){
    //println!("from leetcode/day4/function");
  //  test_104();
  test_102();
}


// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

fn test_104() {
    let depth = max_depth_104(vec!["1","2","3","4"]);
    println!("{:?}",depth);
 }
/**
 * 这个题， 其实不需要 TreeNode， 来表示树， 直接用 数组表示就行， 可以先试一下数组的
 * 最大深度，本身就是 树的高度， 它就是 log(n) +1     n 是节点总数
 *  rust 中如何计算对数的
 */
 fn max_depth_104(tree: Vec<&str>) -> i32{
    let len = tree.len();
    let result = len.ilog2() + 1;

    result as i32
}
/**
 * 如果依然是想用 树节点， 可以使用深度优先遍历
 * 头节点
 * 左节点
 * 右节点
 * 一旦两边都为空就 与max 对比， 这里有一个深度， 
 * treeNode 为什么要用  多owner 与 单改变者呢
 * 下面的写法一下就可以了
 */
fn max_depth_104_v2(root: Option<Rc<RefCell<TreeNode>>>) -> i32{
    
    if let Some(node) = root.clone() {
        let node_borrow = node.borrow();
        let left  = node_borrow.left.clone();
        let right = node_borrow.right.clone();
    
        return (1+ max_depth_104_v2(left)).max(1+ max_depth_104_v2(right));
    }
    0
}


/**
 * v2 和简写
 */
fn max_depth_104_v3(root: Option<Rc<RefCell<TreeNode>>>) -> i32{
    
    if let Some(node) = root.as_ref() {
        let node_borrow = node.borrow();
        let left  = node_borrow.left.clone();
        let right = node_borrow.right.clone();
    
        return (1+ max_depth_104_v2(left)).max(1+ max_depth_104_v2(right));
    }
    0
}

/**
 * v2 的另一简写
 */
fn max_depth_104_v4(root: Option<Rc<RefCell<TreeNode>>>) -> i32{
    
        match(root){
            Some(node) => {
                let node_borrow = node.borrow();
                let left  = node_borrow.left.clone();
                let right = node_borrow.right.clone();
            
                 (1+ max_depth_104_v2(left)).max(1+ max_depth_104_v2(right))
            }
            _ => 0
        }
        
    }

/**
 * 102 题 树节点的层序遍历。  这种题目一旦掌握了 递归， 就好办的， 剩下的就是小小的语法问题了
 *   当前节点，所有的下层节点放于一个 列表  fifo的即可， 记下最终位置， 或者就搞两个， 交替进行
 * 
 */
fn level_order_102(root:Option<Rc<RefCell<TreeNode>>>) ->Vec<Vec<i32>>{
   let mut res= Vec::new();//<vec<i32>>;
   let mut list1 = LinkedList::new();
   let mut list2:LinkedList<Option<Rc<RefCell<TreeNode>>>> = LinkedList::new();

   list1.push_back(root);
   // 2 列表肯定有一个为空
   while( !list1.is_empty() || !list2.is_empty()){


      if(list2.is_empty()){
         let mut int_list = Vec::new();
        for node in list1.clone() {
          
          if let  Some(true_node) = node {
              int_list.push(true_node.borrow().val);
              list2.push_back(true_node.borrow_mut().left.clone());
              list2.push_back(true_node.borrow_mut().right.clone());
          }

        }
        if !int_list.is_empty() {
          res.push(int_list.clone());
        }
        list1.clear();
      }

      if(list1.is_empty()){
        let mut int_list = Vec::new();
       for node in list2.clone() {
         
         if let  Some(true_node) = node {
             int_list.push(true_node.borrow().val);
             list1.push_back(true_node.borrow_mut().left.clone());
             list1.push_back(true_node.borrow_mut().right.clone());
         }

       }
       if !int_list.is_empty() {
        res.push(int_list.clone());
      }
       list2.clear();
     }

      
   }


   res

   

}

/**
 * 使用 leetcode 上的 vecDeque  ， 作为队列，理简洁些
 * //  用LinkedList 也是一样的效果的， 它的想法是 这个队列在循环它时，长度是固定的，，  queue.len()  就算后面加了内容，也不会增长， 除非重新获取。
 */
fn level_order_102_v2(root:Option<Rc<RefCell<TreeNode>>>) ->Vec<Vec<i32>>{

  let mut res = Vec::new();
 // let mut list = LinkedList::new();
 let mut list = VecDeque::new();
  if let Some(node) = root {
    list.push_back(node);
  }else {
    return res;
  }
  
  while !list.is_empty() {
    let mut val_vec = Vec::new();
    for _ in 0..list.len() {
      //let cur_node = list.pop_front();
      let cur_node =list.pop_front().unwrap() ;
        val_vec.push(cur_node.borrow().val);
        if let Some(left) = cur_node.borrow().left.clone() {
          list.push_back(left);
        }
        
        if let Some(right) = cur_node.borrow().right.clone() {
          list.push_back(right);
        }

      
    }

    if !val_vec.is_empty() {
      res.push(val_vec);
    }
    

  }

  res










  // let mut res = vec![];
  // let mut queue = VecDeque::new();
  // if let Some(node) = root {
  //     queue.push_back(node);
  // }
  // while !queue.is_empty() {
  //     let mut itemList = vec![];
  //     for _ in 0..queue.len() {
  //         let tempNode = queue.pop_front().unwrap();
  //         itemList.push(tempNode.borrow().val);
  //         let node_borrow = tempNode.borrow();
  //         if let Some(left) = node_borrow.left.as_ref() {
  //             queue.push_back(Rc::clone(left));
  //         }
  //         if let Some(right) = node_borrow.right.as_ref(){
  //             queue.push_back(Rc::clone(right));
  //         }
  //     }
  //     res.push(itemList);
  // }
  // res

}


fn test_102(){
  let root = TreeNode::new(20);
  let res = level_order_102(Some(Rc::new(RefCell::new(root))));

  print!("{:?}",res);

  
}

/**
 * 这个只是左右子树的 反转， 简单题   ， take 掉， 再放去就好
 * 确定好， 有左 右， 才转， 其实没有也要转的
 */
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    
    match root {
      Some(node) =>{
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        node.borrow_mut().left = invert_tree(right);
        node.borrow_mut().right = invert_tree(left);
        Some(node)
      }
      None => {
        root
      }
    }
         
}
   




