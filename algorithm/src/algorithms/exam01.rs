// 第一题 100 分
// 某公司研发了一款高性能AI处理器。每台物理设备具备8颗AI处理器，编号分别为0、1、2、3、4、5、6、7。

// 编号0-3的处理器处于同一个链路中，编号4-7的处理器处于另外一个链路中，不通链路中的处理器不能通信。

// 如下图所示。现给定服务器可用的处理器编号数组array，以及任务申请的处理器数量num，找出符合下列亲和性调度原则的芯片组合。

// 如果不存在符合要求的组合，则返回空列表。

// 亲和性调度原则：

// -如果申请处理器个数为1，则选择同一链路，剩余可用的处理器数量为1个的最佳，其次是剩余3个的为次佳，然后是剩余2个，最后是剩余4个。

// -如果申请处理器个数为2，则选择同一链路剩余可用的处理器数量2个的为最佳，其次是剩余4个，最后是剩余3个。

// -如果申请处理器个数为4，则必须选择同一链路剩余可用的处理器数量为4个。

// -如果申请处理器个数为8，则申请节点所有8个处理器。

// 提示：

// 任务申请的处理器数量只能是1、2、4、8。
// 编号0-3的处理器处于一个链路，编号4-7的处理器处于另外一个链路。
// 处理器编号唯一，且不存在相同编号处理器。
// 输入描述
// 输入包含可用的处理器编号数组array，以及任务申请的处理器数量num两个部分。

// 第一行为array，第二行为num。例如：

// [0, 1, 4, 5, 6, 7]
// 1

// 表示当前编号为0、1、4、5、6、7的处理器可用。任务申请1个处理器。

// 0 <= array.length <= 8
// 0 <= array[i] <= 7
// num in [1, 2, 4, 8]
// 输出描述
// 输出为组合列表，当array=[0，1，4，5，6，7]，num=1 时，输出为[[0], [1]]。

// 分析， 这个用到的回溯遍历   ， 主要是在  申请个数超1， 但不超一个数组大小的， 需要反复的去做回溯遍历
// 就是从 一组数中， 选出 几个 ， 有哪几种组合选 法
// dfs(arr, start_index,sub_arr_size, sub_arr,res){
//    for i in start_index  .. arr.size() {

//   }
// }

use std::io;

use crate::algorithms::read_helper;

pub fn get_ports() -> Vec<Vec<i32>> {
    println!("以空格， 输入  0 - 8 的处理器编号， 可以断开输入， 以回车结束输入");
  //  let arr = read_helper::read_vec_i32(false);
    let arr = read_helper::read_vec_i32_bracket(false);
    println!("输入的处理器编号为：{:?}", arr);

    // 将输入的数据  0-4 为一个链路， 4-7 为另一个链路， 以此来区分两个链路的处理器数量
    let arr1: Vec<i32> = arr.iter().filter(|&&x| x >= 0 && x <= 3).cloned().collect();
    let arr2: Vec<i32> = arr.iter().filter(|&&x| x >= 4 && x <= 7).cloned().collect();

    let num = read_helper::read_i32(true);
    let mut result = Vec::new();
    match num {
        1 => {
            if arr1.len() == 1 || arr2.len() == 1 {
                if arr1.len() == 1 {
                    result.extend(get_combine_ways(arr1, 1));
                }

                if arr2.len() == 1 {
                    result.extend(get_combine_ways(arr2, 1));
                }
                return result;
            }

            if arr1.len() == 3 || arr2.len() == 3 {
                if arr1.len() == 3 {
                    result.append(&mut get_combine_ways(arr1, 1));
                }

                if arr2.len() == 3 {
                    result.append(&mut get_combine_ways(arr2, 1));
                }

                return result;
            }

            if arr1.len() == 2 || arr2.len() == 2 {
                if arr1.len() == 2 {
                    result.append(&mut get_combine_ways(arr1, 1));
                }

                if arr2.len() == 2 {
                    result.append(&mut get_combine_ways(arr2, 1));
                }

                return result;
            }

            if arr1.len() == 4 || arr2.len() == 4 {
                if arr1.len() == 4 {
                    result.append(&mut get_combine_ways(arr1, 1));
                }

                if arr2.len() == 4 {
                    result.append(&mut get_combine_ways(arr2, 1));
                }

                return result;
            }
        }
        2 => {
            if arr1.len() == 2 || arr2.len() == 2 {
                if arr1.len() == 2 {
                    result.append(&mut get_combine_ways(arr1, 2));
                }

                if arr2.len() == 2 {
                    result.append(&mut get_combine_ways(arr2, 2));
                }

                return result;
            }

            if arr1.len() == 4 || arr2.len() == 4 {
                if arr1.len() == 4 {
                    result.append(&mut get_combine_ways(arr1, 2));
                }

                if arr2.len() == 4 {
                    result.append(&mut get_combine_ways(arr2, 2));
                }

                return result;
            }

            if arr1.len() == 3 || arr2.len() == 3 {
                if arr1.len() == 3 {
                    result.append(&mut get_combine_ways(arr1, 3));
                }

                if arr2.len() == 3 {
                    result.append(&mut get_combine_ways(arr2, 3));
                }

                return result;
            }
        }
        4 => {
            if arr1.len() == 4 || arr2.len() == 4 {
                if arr1.len() == 4 {
                    result.push(arr1);
                }

                if arr2.len() == 4 {
                    result.push(arr2);
                }

                return result;
            }
        }
        8 => {
            if arr1.len() == 4 && arr2.len() == 4 {
                let mut combined = arr1.clone();
                combined.extend(arr2.clone());
                result.push(combined);

                return result;
            }
        }
        _ => {}
    }

    result
}

//
pub fn get_combine_ways(arr: Vec<i32>, num: usize) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut sub_arr: Vec<i32> = vec![];
    let arr_len = arr.len();

    if arr_len < num {
        return result;
    }

    dfs(&arr, arr_len, 0, num, &mut sub_arr, &mut result);
    result
}

/**
 * 调本方法前， 主调方法检查  need_num  <= arr.len()， 否则构成不了
 * arr: 选取用的数组
 * from: 从arr 哪个下标（索引位置）开始
 * need: 选取的数量
 * sub_arr: 已选中的元素组成的子数组
 * result: 选取完成的数组 组成的数组集合
 *  
 * */
fn dfs(
    arr: &Vec<i32>,
    arr_len: usize,
    from_index: usize,
    need_num: usize,
    sub_arr: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    let len = sub_arr.len();

    if len == need_num as usize {
        result.push(sub_arr.clone());
        return;
    }

    for i in from_index..(arr_len) {
        sub_arr.push(arr[i]);
        dfs(arr, arr_len, i + 1, need_num, sub_arr, result);
        // sub_arr.remove( len -1);
        sub_arr.pop();
    }
}

// 题目二：
// 题目描述
// VLAN是一种对局域网设备进行逻辑划分的技术，为了标识不同的VLAN，引入VLAN ID(1-4094之间的整数)的概念。

// 定义一个VLAN ID的资源池(下称VLAN资源池)，资源池中连续的VLAN用开始VLAN-结束VLAN表示，不连续的用单个整数表示，所有的VLAN用英文逗号连接起来。

// 现在有一个VLAN资源池，业务需要从资源池中申请一个VLAN，需要你输出从VLAN资源池中移除申请的VLAN后的资源池。

// 输入描述
// 第一行为字符串格式的VLAN资源池，第二行为业务要申请的VLAN，VLAN的取值范围为[1,4094]之间的整数。

// 输出描述
// 从输入VLAN资源池中移除申请的VLAN后字符串格式的VLAN资源池，输出要求满足题目描述中的格式，并且按照VLAN从小到大升序输出。
// 如果申请的VLAN不在原VLAN资源池内，输出原VLAN资源池升序排序后的字符串即可。

// 示例1
// 输入

// 1-5
// 2
// 1
// 2
// 输出

// 1,3-5
// 1
// 说明

// 原VLAN资源池中有VLAN 1、2、3、4、5，从资源池中移除2后，剩下VLAN 1、3、4、5，按照题目描述格式并升序后的结果为1,3-5

// 示例2
// 输入

// 20-21,15,18,30,5-10
// 15
// 1
// 2
// 输出

// 5-10,18,20-21,30
// 1
// 说明

// 原VLAN资源池中有VLAN 5、6、7、8、9、10、15、18、20、21、30，从资源池中移除15后，资源池中剩下的VLAN为 5、6、7、8、9、10、18、20、21、30，按照题目描述格式并升序后的结果为5-10,18,20-21,30。

// 示例3
// 输入

// 5,1-3
// 10
// 1
// 2
// 输出

// 1-3,5
// 1
// 说明

// 原VLAN资源池中有VLAN 1、2、3，5，申请的VLAN 10不在原资源池中，将原资源池按照题目描述格式并按升序排序后输出的结果为1-3,5。

// 分析 ：  接受第一行字符串，排序。   第二为接收一个数字。

pub fn get_vans() {
    let stdin = io::stdin();
    let mut input = stdin.lines();

    // 第一行： 可用 van 资源池
    println!("请输入 vlan 资源池，格式为 1-5,7,9-10 这样的格式， 以回车结束输入");
    let vlan_pool_str = input.next().unwrap().unwrap().trim().to_string();
    println!("你输入的 vlan 资源池为： {}", vlan_pool_str);
    // 第二行： 申请的 vlan
    let van_seq: i32 = input.next().unwrap().unwrap().trim().parse().unwrap();

    // let result = remove_van(vlan_pool_str, van_seq);
    let result = remove_van_v2(vlan_pool_str, van_seq); // 采用std::collections::BTreeMap 来处理， 这样就不需要在最后转换回字符串格式了， 直接在 map 中进行处理， 最后输出即可。
    println!("取走{}后的序列为{}", van_seq, result);
}

fn remove_van(vlan_pool_str: String, van_seq: i32) -> String {
    // 解析 vlan_pool_str， 将其转换为一个包含所有 VLAN ID 的 Vec<i32>
    let mut vlan_ids = Vec::new();
    for part in vlan_pool_str.split(',') {
        if part.contains('-') {
            let bounds: Vec<&str> = part.split('-').collect();
            let start: i32 = bounds[0].parse().unwrap();
            let end: i32 = bounds[1].parse().unwrap();
            for id in start..=end {
                vlan_ids.push(id);
            }
        } else {
            vlan_ids.push(part.parse().unwrap());
        }
    }

    // 从 vlan_ids 中移除 van_seq
    vlan_ids.retain(|&id| id != van_seq);

    // 将剩余的 VLAN ID 转换回字符串格式
    vlan_ids.sort_unstable();
    let mut result = String::new();
    let mut i = 0;
    while i < vlan_ids.len() {
        let start = vlan_ids[i];
        let mut end = start;
        // 用来判断 连续  就是下一个节点 正是 end+1。
        while i + 1 < vlan_ids.len() && vlan_ids[i + 1] == end + 1 {
            end = vlan_ids[i + 1];
            i += 1;
        }
        if !result.is_empty() {
            result.push(',');
        }
        if start == end {
            result.push_str(&start.to_string());
        } else {
            result.push_str(&format!("{}-{}", start, end));
        }
        i += 1;
    }

    result
}

// 上述方法解析输入的 VLAN 资源池字符串，生成一个包含所有 VLAN ID 的 Vec<i32>，然后从中移除申请的 VLAN ID，最后将剩余的 VLAN ID 转换回字符串格式输出。
// 还可以按我的老路子想法，将其处理好， 开始结束封装为 map， (key,value) 即 (start,end)  这样就不需要在最后转换回字符串格式了， 直接在 map 中进行处理， 最后输出即可。  但是上面的方法更简单一些， 就是先把字符串解析成一个数组， 然后再从数组中移除申请的 vlan id， 最后再把剩余的 vlan id 转换回字符串格式输出。
// std::collections::BTreeMap 是一个有序的 map， 可以保证输出的时候是按照 vlan id 从小到大升序输出的。不是插入的时间先后
fn remove_van_v2(vlan_pool_str: String, van_seq: i32) -> String {
    let mut vlan_map = std::collections::BTreeMap::new(); // 它的排序规则是按照 key 的升序排序的， 这样就可以保证输出的时候是按照 vlan id 从小到大升序输出的。不是插入的时间先后
    for part in vlan_pool_str.split(',') {
        if part.contains('-') {
            let bounds: Vec<&str> = part.split('-').collect();
            let start: i32 = bounds[0].parse().unwrap();
            let end: i32 = bounds[1].parse().unwrap();
            vlan_map.insert(start, end);
        } else {
            let id: i32 = part.parse().unwrap();
            vlan_map.insert(id, id);
        }
    }

    // 从 vlan_map 中移除 van_seq
    for (&start, &end) in &vlan_map {
        if van_seq >= start && van_seq <= end {
            vlan_map.remove(&start);
            if van_seq > start {
                vlan_map.insert(start, van_seq - 1);
            }
            if van_seq < end {
                vlan_map.insert(van_seq + 1, end);
            }
            break;
        }
    }

    // 将剩余的 VLAN ID 转换回字符串格式
    let mut result = String::new();
    for (&start, &end) in &vlan_map {
        if !result.is_empty() {
            result.push(',');
        }
        if start == end {
            result.push_str(&start.to_string());
        } else {
            result.push_str(&format!("{}-{}", start, end));
        }
    }

    result
}
