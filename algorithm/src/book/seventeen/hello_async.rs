use core::task;
use std::{thread, time::Duration};

use trpl::{Either, Html};

use crate::algorithms::test;

pub fn sub_main() {
    // single_title();
    // double_title();

    // spaw_print();
    // thread_print();
    // async_print();
    // async_channel();
    // async_channel_interval();
    // yield_to_runtime();
    // yield_to_runtime2();
    // yield_to_runtime3();
    // yield_to_runtime4(); // yield_now
    println!("in hello_async");
    // test_timeout();
    test_pin();
}

fn single_title() {
    println!("single_title");

    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}

fn double_title() {
    println!("single_title");

    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let title_fut_1 = page_title2(&args[1]);
        let title_fut_2 = page_title2(&args[2]);

        let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url}  return first");
        match maybe_title {
            Some(title) => println!("The page title was {title}"),
            None => println!(" it has no title"),
        }
    })
}

// 当看到一个block有 async 时， 它会将它转化为唯一的， 匿名的实现了 Future trait的 数据 类型，
// 当看到一个function有 async 时， 它会将它转化为 一个非异步的函数， 它的函数体是 async 的block,

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

fn page_title_sync(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        // 没有move 也不会报错， 那move的作用是什么？
        let response = trpl::get(url).await;
        let response_text = response.text().await;
        Html::parse(&response_text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}

async fn page_title2(url: &str) -> (&str, Option<String>) {
    println!("in function :page_title2 ; the url is {url}");
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

// 在这个函数中， 可以看到如果主线程事完了， 没有采取等待措施， 子线程中事情没做完， 也会直接停掉， 如下， 子线程打印不完.   这里可能不叫线程， 而叫
// 好像主线程与子线程的速度，并不多的。
// 他们在同一个异步代码块中， 只是 第2个在主线程中，
fn spaw_print() {
    trpl::block_on(async {
        let task_one_handler = trpl::spawn_task(async {
            for i in 0..10 {
                println!("first task seq: {i} ");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 0..5 {
            println!("second task seq: {i} ");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        task_one_handler.await.unwrap();
    });
}
//  the task spawned by spawn_task is shut down when the main function ends

fn thread_print() {
    println!("spawn  a thread to print");

    let task1 = thread::spawn(|| {
        for i in 0..10 {
            println!("first task seq: {i} ");
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 0..5 {
        println!("second task seq: {i} ");
        thread::sleep(Duration::from_millis(500));
    }

    task1.join().unwrap();
}

fn async_print() {
    println!("two async !");

    trpl::block_on(async {
        let fut1 = async {
            for i in 0..10 {
                println!("1st task seq: {i} ");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 0..5 {
                println!("2nd task seq: {i} ");
                trpl::sleep(Duration::from_millis(1000)).await;
            }
        };

        // for i in 0..3 {
        //     println!("3rd task seq: {i} ");
        //     thread::sleep(Duration::from_micros(500));
        // }
        trpl::join(fut1, fut2).await;
    });

    // 异步任务 会先执行， 这个最后执行， 有点顺序， 感觉像是不同的运行时， 在操作
    // for i in 0..3 {
    //     println!("3rd task seq: {i} ");
    //     thread::sleep(Duration::from_micros(500));
    // }
}

// std  下的channel 是同步阻塞的。 这里异步的， 是非阻塞的
fn async_channel() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        // let val = String::from("hi");
        // tx.send(val).unwrap(); // 类型第一次定好就定好了， 后面不能再定的。

        // //tx.send(1258).unwrap();
        // let received = rx.recv().await.unwrap();
        // println!("received  '{received}'");

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(1000)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    })
}

fn async_channel_interval() {
    println!("should send intervally");
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("message"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        //   trpl::join(tx_fut, rx_fut).await;
        trpl::join!(tx_fut, tx1_fut, rx_fut);
    })
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{name}' process {ms} ms!");
}

// slow 里面的sleep 是同步的， 所以一个一个的等，  顺序的， 但  trpl::sleep 是异步的， finished 也间隔了。
// 输出 a' started
// a' process 300 ms!
// a' process 100 ms!
// a' process 200 ms!
//  b' started
// b' process 200 ms!
// b' process 100 ms!
// b' process 3500 ms!
//  a' finished
//  b' finished

fn yield_to_runtime() {
    trpl::block_on(async {
        let a = async {
            println!(" a' started ");
            slow("a", 300);
            slow("a", 100);
            slow("a", 200);
            trpl::sleep(Duration::from_millis(1000)).await;
            println!(" a' finished ");
        };

        let b = async {
            println!(" b' started ");
            slow("b", 200);
            slow("b", 100);
            slow("b", 3500);
            trpl::sleep(Duration::from_millis(1000)).await;
            println!(" b' finished ");
        };

        trpl::join(a, b).await;
    });
}

// 去除两个     trpl::sleep  会怎样？
// 还是整齐的打印的
// a' started
// a' process 300 ms!
// a' process 100 ms!
// a' process 200 ms!
//  a' finished
//  b' started
// b' process 200 ms!
// b' process 100 ms!
// b' process 3500 ms!
//  b' finished

fn yield_to_runtime2() {
    println!("without trpl::sleep");

    trpl::block_on(async {
        let a = async {
            println!(" a' started ");
            slow("a", 300);
            slow("a", 100);
            slow("a", 200);
            println!(" a' finished ");
        };

        let b = async {
            println!(" b' started ");
            slow("b", 200);
            slow("b", 100);
            slow("b", 3500);
            println!(" b' finished ");
        };

        trpl::join(a, b).await;
    });
}

fn yield_to_runtime3() {
    println!("gap one trpl::sleep");
    let one_millis = Duration::from_millis(1);

    trpl::block_on(async {
        let a = async {
            println!(" a' started ");
            slow("a", 300);
            trpl::sleep(one_millis).await;
            slow("a", 100);
            trpl::sleep(one_millis).await;
            slow("a", 200);
            trpl::sleep(one_millis).await;
            println!(" a' finished ");
        };

        let b = async {
            println!(" b' started ");
            slow("b", 200);
            trpl::sleep(one_millis).await;
            slow("b", 100);
            trpl::sleep(one_millis).await;
            slow("b", 3500);
            trpl::sleep(one_millis).await;
            println!(" b' finished ");
        };

        trpl::join(a, b).await;
    });
}

fn yield_to_runtime4() {
    println!("gap one trpl::yield_now");
    let one_millis = Duration::from_millis(1);

    trpl::block_on(async {
        let a = async {
            println!(" a' started ");
            slow("a", 300);
            trpl::yield_now().await;
            slow("a", 100);
            trpl::yield_now().await;
            slow("a", 200);
            trpl::yield_now().await;
            println!(" a' finished ");
        };

        let b = async {
            println!(" b' started ");
            slow("b", 200);
            trpl::yield_now().await;
            slow("b", 100);
            trpl::yield_now().await;
            slow("b", 3500);
            trpl::yield_now().await;
            println!(" b' finished ");
        };

        trpl::join(a, b).await;
    });
}

// build our own Async  abstraction
async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn test_timeout() {
    println!("test_timeout,,,,,");
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("succeeded with  '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs());
            }
        }
    });
}

fn test_pin() {
    println!("test_ping function , to describe the pin concept");
    trpl::block_on(async {
        let mut x = 5;
        let mut y = 10;

        let x_ref = &mut x;
        let y_ref = &mut y;

        println!("Before pinning: x = {}, y = {}", x_ref, y_ref);

        // Pinning the references
        let pinned_x = std::pin::Pin::new(x_ref);
        let pinned_y = std::pin::Pin::new(y_ref);

        // Now we cannot move the pinned references, but we can still access their values
        println!("After pinning: x = {}, y = {}", pinned_x, pinned_y);
    });
}
