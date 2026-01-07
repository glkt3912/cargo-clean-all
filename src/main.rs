mod cleaner;
mod config;
mod logger;
mod notify;
mod scanner;

use clap::Parser;
use config::Config;
use logger::Logger;

#[derive(Parser, Debug)]
#[command(name = "cargo-clean-all")]
#[command(about = "Clean all Rust target directories", version)]
struct Args {
    /// Dry run mode (don't actually delete)
    #[arg(short, long)]
    dry_run: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Custom config file path
    #[arg(short, long)]
    config: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // 設定読み込み
    let config = Config::load()?;
    let logger = Logger::new(&config.logging.log_file);

    logger.log_info("Starting cargo-clean-all");

    if args.verbose {
        println!("📋 Configuration:");
        println!("   Scan roots: {:?}", config.paths.scan_roots);
        println!("   Exclude dirs: {:?}", config.paths.exclude_dirs);
        println!("   Notifications: {}", config.notification.enabled);
        println!();
    }

    // スキャン実行
    let mut total_size = 0u64;
    let mut all_targets = Vec::new();
    let mut missing_paths = Vec::new();

    for root in &config.paths.scan_roots {
        if args.verbose {
            println!("🔍 Scanning {}...", root);
        }

        match scanner::scan_target_directories(root, &config.paths.exclude_dirs) {
            scanner::ScanResult::Success(targets) => {
                if args.verbose {
                    println!("   Found {} target directories", targets.len());
                }
                all_targets.extend(targets);
            }
            scanner::ScanResult::PathNotFound(path) => {
                missing_paths.push(path.clone());
                logger.log_error(&format!("Path not found: {}", path));
                eprintln!("⚠️  Warning: Path not found: {}", path);
            }
        }
    }

    // マウントされていないディスクがある場合
    if !missing_paths.is_empty() {
        let message = format!(
            "Some scan paths are not accessible:\n{}",
            missing_paths.join("\n")
        );

        if config.notification.enabled {
            notify::send_notification("Cargo Clean All - Warning", &message);
        }

        if !args.dry_run {
            println!("\n⚠️  Warning: {} path(s) not accessible", missing_paths.len());
            println!("This may happen if external drives are not mounted.");
        }

        // パスが見つからない場合は、他のパスがあればそれを続行
        if all_targets.is_empty() {
            println!("\n⚠️  No paths available to scan. Exiting.");
            logger.log_error("No paths available to scan");
            return Ok(());
        }
    }

    // サイズ計算
    for target in &all_targets {
        total_size += target.size_bytes;
    }

    let size_mb = total_size as f64 / 1024.0 / 1024.0;
    let total_count = all_targets.len();

    if total_count == 0 {
        println!("✨ No target directories found.");
        logger.log_info("No target directories found");
        return Ok(());
    }

    if args.dry_run {
        println!("🔍 Dry run mode - no files will be deleted\n");
        println!("Found {} target directories:", total_count);
        for target in &all_targets {
            let mb = target.size_bytes as f64 / 1024.0 / 1024.0;
            println!("  {} ({:.2} MB)", target.path.display(), mb);
        }
        println!("\n📊 Total size: {:.2} MB", size_mb);
        logger.log_info(&format!(
            "Dry run: {} targets, {:.2} MB",
            total_count, size_mb
        ));
        return Ok(());
    }

    // クリーンアップ実行
    println!("🧹 Cleaning {} target directories...", total_count);
    let mut success_count = 0;
    let mut error_count = 0;
    let mut freed_bytes = 0u64;

    for target in &all_targets {
        if args.verbose {
            print!("  Cleaning {}... ", target.path.display());
        } else {
            print!("  [{}/{}] ", success_count + error_count + 1, total_count);
        }

        let result = cleaner::clean_target(&target.path, target.size_bytes, false);

        if result.success {
            println!("✅");
            success_count += 1;
            freed_bytes += result.size_freed;
            logger.log_info(&format!("Cleaned: {}", result.path));
        } else {
            println!("❌");
            error_count += 1;
            if let Some(err) = &result.error {
                logger.log_error(&format!("Error cleaning {}: {}", result.path, err));
                if args.verbose {
                    eprintln!("    Error: {}", err);
                }
            }
        }
    }

    let freed_mb = freed_bytes as f64 / 1024.0 / 1024.0;

    // 結果表示
    println!("\n✨ Cleanup complete!");
    println!("  Success: {}", success_count);
    println!("  Errors: {}", error_count);
    println!("  Freed: {:.2} MB", freed_mb);

    logger.log_info(&format!(
        "Cleanup complete: {} success, {} errors, {:.2} MB freed",
        success_count, error_count, freed_mb
    ));

    // 通知
    if config.notification.enabled {
        if config.notification.error_only && error_count == 0 {
            // エラーのみ通知モードで、エラーがない場合は通知しない
        } else {
            let message = if error_count > 0 {
                format!(
                    "Cleaned {} dirs with {} errors. Freed {:.2} MB",
                    success_count, error_count, freed_mb
                )
            } else {
                format!("Cleaned {} directories. Freed {:.2} MB", success_count, freed_mb)
            };
            notify::send_notification(&config.notification.title, &message);
        }
    }

    if error_count > 0 {
        std::process::exit(1);
    }

    Ok(())
}
