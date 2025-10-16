use anyhow::Result;
use rocksdb::{Options, DB};

pub fn init(cfg: &crate::config::Config) -> Result<()> {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    std::fs::create_dir_all(&cfg.db_path)?;
    let _db = DB::open(&opts, &cfg.db_path)?;
    Ok(())
}
