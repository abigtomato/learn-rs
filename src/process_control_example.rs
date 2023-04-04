// 流程控制
#[cfg(test)]
mod tests {

    #[test]
    fn if_test() {
        // 条件语句
        let l = 3;
        let number = if l > 0 { 1 } else { -1 };
        println!("{}", number);
    }

    #[test]
    fn while_test() {
        // while循环
        let mut number = 1;
        while number != 4 {
            print!("{}\t", number);
            number += 1;
        }
        println!();
    }

    #[test]
    fn foreach_test() {
        // foreach循环
        let arr = [10, 20, 30, 40, 50];
        for i in arr.iter() {
            print!("{}\t", i);
        }
        println!();
    }

    #[test]
    fn fori_test() {
        // fori循环
        let arr = [10, 20, 30, 40, 50];
        for i in (0..5).rev() {
            print!("{} = {}\t", i, arr[i]);
        }
        println!();
    }

    #[test]
    fn loop_test() {
        // loop循环&循环跳出
        let arr = ['R', 'U', 'N', 'O', 'O', 'B'];
        let mut index = 0;
        let location = loop {
            let ch = arr[index];
            if ch == 'N' {
                break index;
            }
            index += 1;
        };
        println!("{}", location);
    }

    #[test]
    fn break_test() {
        // loop循环&嵌套循环跳出
        let mut count = 0;
        'COUNTING_UP: loop {
            let mut remaining = 10;
            loop {
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'COUNTING_UP;
                }
                remaining -= 1;
            }
            count += 1;
        }
        println!("{}", count);
    }
}
