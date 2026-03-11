use std::collections::{BTreeMap, VecDeque};

use disruptor::{BusySpin, Producer, Sequence, build_single_producer};
use openraft::{Raft, RaftLogReader, RaftSnapshotBuilder, RaftTypeConfig, declare_raft_types};
use rocksdb::DB;
use rust_decimal::{Decimal, prelude::Zero};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
    IOC,           // Immediate or Cancel
    FOK,           // Fill or Kill
    PostOnly,      // Trailing Stop
    Iceberg,       // Hidden order
    TWAP,          // Time-Weighted Average Price
    VWAP,          // Volume-Weighted Average Price
    Pegged,        // Pegged to market price
    OCO,           // One-Cancels-the-Other
    GTC,           // Good-Til-Canceled
    GTD,           // Good-Til-Date
    MarketOnOpen,  // Market order executed at the opening of the trading session
    MarketOnClose, // Market order executed at the closing of the trading session
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Direction {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Order {
    id: u64,
    order_type: OrderType,
    symbol: String,
    quantity: Decimal,
    price: Option<Decimal>, // Price is optional for market orders
    timestamp: u64,         // Unix timestamp in milliseconds
    direction: Direction,
}

#[derive(Debug)]
enum Event {
    NewOrder(Order),
    CancelOrder(u64), // Order ID
    Trade {
        buy_order_id: String,
        sell_order_id: String,
        price: Decimal,
        quantity: Decimal,
    },
    OrderFilled(u64),          // Order ID
    OrderPartiallyFilled(u64), // Order ID
    OrderCanceled(u64),        // Order ID
    PriceUpdate {
        symbol: String,
        price: Decimal,
    },
}

#[derive(Debug, Clone)]
struct OrderBook {
    bids: BTreeMap<Decimal, VecDeque<Order>>, // Price -> List of buy orders  , buy ,price desc
    asks: BTreeMap<Decimal, VecDeque<Order>>, // Price -> List of sell orders  , sell, price asc
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Trade {
    buy_order_id: u64,
    sell_order_id: u64,
    price: Decimal,
    quantity: Decimal,
}

impl OrderBook {
    fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        let price = order.price.unwrap_or(Decimal::zero());
        match order.direction {
            Direction::Buy => {
                self.bids
                    .entry(price)
                    .or_insert_with(VecDeque::new)
                    .push_back(order);
            }
            Direction::Sell => {
                self.asks
                    .entry(price)
                    .or_insert_with(VecDeque::new)
                    .push_back(order);
            }
        }
    }

    // Additional methods for matching orders, canceling orders, etc. would go here
    fn match_orders(&mut self) -> Vec<Trade> {
        let mut trades = Vec::new();
        // Matching logic would go here, iterating through bids and asks to find matches
        // For simplicity, this is just a placeholder
        trades
    }
}

#[derive(Debug, Clone)]
struct MatchingActor {
    // order book
    book: OrderBook,
    symbol: String,
    // event queue
}

impl MatchingActor {
    fn new(symbol: String) -> Self {
        Self {
            book: OrderBook::new(),
            symbol,
        }
    }

    async fn handle_event(&mut self, event: Event) {
        match event {
            Event::NewOrder(order) => self.book.add_order(order),
            // Handle other events like CancelOrder, Trade, etc.
            _ => unimplemented!(),
        }
    }

    async fn run(&mut self) {
        // Main loop to process events from the event queue
        // For simplicity, this is just a placeholder
        let factory = || {
            Event::NewOrder(Order {
                id: 0,
                order_type: OrderType::Limit,
                symbol: self.symbol.clone(),
                quantity: Decimal::new(100, 0),
                price: Some(Decimal::new(10, 0)),
                timestamp: 0,
                direction: Direction::Buy,
            })
        };

        let symbol = self.symbol.clone();
        let mut book = self.book.clone(); // clone and move into the closure, the original book will not be updated, this is just for demonstration, in real implementation, we need to use Arc<Mutex<OrderBook>> or other concurrency primitives to share the order book between threads
        // another way is to use the actor model, each actor has its own order book and process events sequentially, this way we can avoid the concurrency issues, but we need to implement the message passing between actors, and also need to consider the performance and scalability of the system, for example, we can use a thread pool to run multiple actors in parallel, and use a message queue to communicate between actors, this way we can achieve high throughput and low latency, but we need to carefully design the system architecture and choose the right tools and libraries to implement it.

        let processor = move |e: &Event, seq: Sequence, end: bool| {
            println!("Processing event: {:?}, sequence: {}, end: {}", e, seq, end);
            let trades = book.match_orders();

            if end {
                for trade in trades {
                    println!("Trade executed: {:?}", trade);
                    // push to the kafka or other message queue
                }
            }
        };

        let mut producer = build_single_producer(65536, factory, BusySpin)
            .handle_events_with(processor)
            .build();

        // 有多少个事件， 去处理一下。  对于处理好的trade  ,

        // producer.publish(event);
    }
}

async fn main() {
    let mut handles = Vec::new();
    let symbols = vec!["BTC/ETH", "SOL/BTC", "USDC/BTC"];
    for symbol in symbols {
        let mut actor = MatchingActor::new(symbol.to_string());
        let handle = tokio::spawn(async move {
            actor.run().await;
        });
        handles.push(handle);
    }

    // RAft 集成
}

// declare_raft_types!(
//    // Command = String, // Replace with the appropriate type for your use case
//     NodeId = u64,
//     Node = String,
//     Entry = Event,
//     Snapshot = (),

//   );

struct RocksStorage {
    // Implement the necessary fields for your storage
    db: DB,
}

// impl RaftLogReader<RaftTypeConfig>  for RocksStorage {
//     fn get_log_entries(&self, start: u64, end: u64) -> Vec<Event> {
//         // Implement logic to read log entries from RocksDB
//         vec![]
//     }

//     fn get_log_entry(&self, index: u64) -> Option<Event> {
//         // Implement logic to read a single log entry from RocksDB
//         None
//     }

//     fn get_last_log_index(&self) -> u64 {
//         // Implement logic to get the last log index from RocksDB
//         0
//     }
// }   

// impl RaftSnapshotBuilder<RaftTypeConfig> for RocksStorage {
//     fn build_snapshot(&self) -> () {
//         // Implement logic to build a snapshot from the current state of the order book
//     }
// }   


// 启动Raft
// let cfg  =  opencraft::Config {

// }

// cfg.storage = Arc::new(RocksStorage {
//     db: DB::open_default("path/to/rocksdb")?,
// });

// let raft = Raft::new(cfg, Arc::new(MyStateMachine {})).await?;
