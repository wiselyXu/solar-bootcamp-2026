use core::task;
use std::{thread, time::Duration};

use trpl::{Either, Html};

pub fn sub_main() {
    // single_title();
    // double_title();

    // spaw_print();
    // thread_print();
    // async_print();
    // async_channel();
    async_channel_interval();
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
