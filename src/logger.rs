use std::io::Write;

pub fn init() {
    env_logger::builder()
        .parse_env(env_logger::Env::new().default_filter_or("info"))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Stdout)
        .init();
}
