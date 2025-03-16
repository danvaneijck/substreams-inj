mod pb;
use pb::trades::v1::ChoiceTrades;
use prost::Message;
use sha2::{Digest, Sha256};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use std::time::{Duration, UNIX_EPOCH};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

#[substreams::handlers::map]
fn db_out(input: ChoiceTrades) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = DatabaseChangeTables::new();
    substreams::skip_empty_output();

    // Iterate over each trade in the repeated field.
    for trade in input.trades {
        // Create a unique ID by hashing the encoded trade message.
        let mut hasher = Sha256::new();
        let mut buf = Vec::new();
        trade.encode(&mut buf)?;
        hasher.update(&buf);
        let id = format!("{:x}", hasher.finalize());

        // Create a new row in the "ChoiceTrades" table using the generated id.
        let row = tables.create_row("ChoiceTrades", id);
        row.set("transaction_hash", trade.transaction_hash);
        row.set("contract_address", trade.contract_address);
        row.set("action", trade.action);
        row.set("ask_asset", trade.ask_asset);
        row.set("burn_amount", trade.burn_amount);
        row.set("commission_amount", trade.commission_amount);
        row.set("fee_wallet_amount", trade.fee_wallet_amount);
        row.set("offer_amount", trade.offer_amount);
        row.set("offer_asset", trade.offer_asset);
        row.set("pool_amount", trade.pool_amount);
        row.set("receiver", trade.receiver);
        row.set("return_amount", trade.return_amount);
        row.set("sender", trade.sender);
        row.set("spread_amount", trade.spread_amount);
        row.set("msg_index", trade.msg_index);

        // Set the new block number and block time fields.
        row.set("block_number", trade.block_number.to_string());

        if let Some(block_time) = trade.block_time {
            let sys_time = UNIX_EPOCH + Duration::new(block_time.seconds as u64, block_time.nanos as u32);
            let odt = OffsetDateTime::from(sys_time);
            let ts_str = odt.format(&Rfc3339).unwrap();
            row.set("block_time", ts_str);
        } else {
            row.set("block_time", "");
        }

    }

    Ok(tables.to_database_changes())
}
