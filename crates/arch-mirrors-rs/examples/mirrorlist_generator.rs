use std::process;

const SUCCESS: i32 = 0;
const ERROR: i32 = 1;

#[tokio::main]
async fn main() {
    async fn inner() -> i32 {
        let status = match arch_mirrors_rs::get_status().await {
            Ok(status) => status,
            Err(error) => {
                eprintln!("error: {}", error);
                return ERROR;
            }
        };

        println!(
            r#"##
## Arch Linux repository mirrorlist
## Created by arch_mirrors
## Generated on {}
##
"#,
            chrono::Utc::now().date()
        );

        for url in status.urls {
            println!("## {}", url.country.kind);
            println!("#Server = {}$repo/os/$arch", url.url)
        }

        SUCCESS
    }

    process::exit(inner().await)
}
