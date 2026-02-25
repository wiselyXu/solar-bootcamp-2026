use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    // open a connection to the mini-redis server
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();
    // set a key
    client.set("salary", "45000".into()).await.unwrap();

    // get the key
    let name = client.get("name").await?;
    let salary = client.get("salary").await?;

    println!(
        "the salary of {:?} is {:?} ",
        String::from_utf8(name.unwrap().to_vec()).unwrap(),
        salary.unwrap()
    );

    // got value from the server; result=Some(b"world")  // 去some ， 只需要  unwrap() 就行了， 去b"world"，
    // 需要 String::from_utf8() 来转换成字符串 , 还要转为 Vec(u8)， 因为 b"world" 是一个字节数组， 需要转换成字符串才能打印出来,
    // 如何将b"word" 转为 Vec<u8>， 可以使用 to_vec() 方法， 例如： b"world".to_vec() 就可以将 b"world" 转换成 Vec<u8>，
    // 然后再使用 String::from_utf8() 方法将 Vec<u8> 转换成 String， 例如： String::from_utf8(b"world".to_vec()).unwrap() 就可以将 b"world" 转换成 String。
    // 最终展示为 ： the salary of "xugang" is b"45000"
    Ok(())
}
