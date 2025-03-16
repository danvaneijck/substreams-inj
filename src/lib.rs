mod pb;
use pb::{trades::v1 as trades, sf::substreams::cosmos::v1::EventList};

#[substreams::handlers::map]
fn choice_trades(events: EventList) -> trades::ChoiceTrades {
    let mut output = trades::ChoiceTrades::default();

    // Extract clock info (if available) from the EventList.
    let (block_number, block_time) = if let Some(clock) = events.clock {
        (clock.number, clock.timestamp)
    } else {
        (0, None)
    };

    // Iterate over all events in the EventList.
    for evt in events.events {
        // You might want to filter by event type if needed.
        if let Some(inner_event) = evt.event {
            if inner_event.r#type != "wasm" {
                continue;
            }

            let tx_hash = evt.transaction_hash;
            // Create a new Trade record.
            let mut trade = trades::Trade::default();
            trade.transaction_hash = tx_hash;

            // Iterate over attributes and extract the desired data.
            for attr in inner_event.attributes {
                match attr.key.as_str() {
                    "_contract_address" => trade.contract_address = attr.value,
                    "action" => trade.action = attr.value,
                    "ask_asset" => trade.ask_asset = attr.value,
                    "burn_amount" => trade.burn_amount = attr.value,
                    "commission_amount" => trade.commission_amount = attr.value,
                    "fee_wallet_amount" => trade.fee_wallet_amount = attr.value,
                    "offer_amount" => trade.offer_amount = attr.value,
                    "offer_asset" => trade.offer_asset = attr.value,
                    "pool_amount" => trade.pool_amount = attr.value,
                    "receiver" => trade.receiver = attr.value,
                    "return_amount" => trade.return_amount = attr.value,
                    "sender" => trade.sender = attr.value,
                    "spread_amount" => trade.spread_amount = attr.value,
                    "msg_index" => trade.msg_index = attr.value,
                    _ => {},
                }
            }

            trade.block_number = block_number;
            trade.block_time = block_time;

            // Add the trade to our output.
            output.trades.push(trade);
        }
    }
    output
}
