#![no_std]
#![no_main]

use {defmt_rtt as _, embassy_rp as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // if brackets commented bss size: 41344 else 1344
    // {
    let res: Result<(), [u8; 40000]> = Ok(());
    assert!(res.is_ok());
    // }
    embassy_futures::yield_now().await;
}
