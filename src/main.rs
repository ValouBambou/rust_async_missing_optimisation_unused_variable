fn main() {
    dbg!(size_of_val(&bar()));
    assert!(size_of_val(&bar()) < 40_000);
}
async fn bar() {
    // if brackets commented size: 40003 else 2
    // {
    let res: Result<(), [u8; 40000]> = Ok(());
    assert!(res.is_ok());
    // }
    foo().await;
}
async fn foo() {}
