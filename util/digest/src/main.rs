#[bakkie_simple::tool("sha256")]
fn sha256(name: String) {}

#[bakkie_simple::tool("sha256")]
fn find_last_ten_logs(name: String) {}

fn main() -> bakkie_simple::Result<()> {
    bakkie_simple::run()
}
