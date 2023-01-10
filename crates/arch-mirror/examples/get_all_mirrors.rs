use std::process;

#[tokio::main]
async fn main() {
    const SUCCESS: i32 = 0;
    const ERROR: i32 = 1;

    async fn inner() -> i32 {
        let status = match arch_mirror::get_status().await {
            Ok(status) => status,
            Err(error) => {
                eprintln!("error: {}", error);
                return ERROR;
            }
        };

        for mirror in status.urls {
            println!("{}", mirror.url)
        }

        SUCCESS
    }

    process::exit(inner().await)
}
