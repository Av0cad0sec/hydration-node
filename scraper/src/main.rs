use clap::Parser;
use hydradx_runtime::Block;
use remote_externalities::*;
use sp_runtime::traits::Block as BlockT;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct StorageCmd {
	/// The block hash at which to get the runtime state. Will be latest finalized head if not
	/// provided.
	#[clap(long)]
	at: Option<<Block as BlockT>::Hash>,
	/// The pallets to scrape. If empty, entire chain state will be scraped.
	#[clap(long, multiple_values = true)]
	pallet: Vec<String>,
	#[allow(missing_docs)]
	#[clap(flatten)]
	shared: SharedParams,
}

#[derive(Parser, Debug)]
struct BlocksCmd {
	/// The block number of the first block that will be stored.
	from: u32,
	/// The number of blocks.
	num_of_blocks: u32,
	#[allow(missing_docs)]
	#[clap(flatten)]
	shared: SharedParams,
}

/// Possible commands of `scraper`.
#[derive(Parser, Debug)]
enum Command {
	SaveStorage(StorageCmd),
	SaveBlocks(BlocksCmd),
}

/// Shared parameters of the `scraper` commands.
#[derive(Parser, Debug)]
struct SharedParams {
	/// The url to connect to.
	#[clap(short, long)]
	uri: String,
	/// The path where to save the storage file.
	#[clap(long)]
	path: Option<PathBuf>,
}

impl SharedParams {
	fn get_path(&self) -> PathBuf {
		if let Some(mut maybe_path) = self.path.clone() {
			maybe_path.push(STORAGE_FILE_NAME);
			maybe_path
		} else {
			let mut curr_dir = PathBuf::new();
			curr_dir.push(file!());
			curr_dir.pop();
			curr_dir.push("..");
			curr_dir.set_file_name(STORAGE_FILE_NAME);
			curr_dir
		}
	}
}

#[derive(Parser, Debug)]
struct Cli {
	#[clap(subcommand)]
	command: Command,
}

pub const STORAGE_FILE_NAME: &str = "SNAPSHOT";

fn main() {
	let args = Cli::parse();

	let path = match args.command {
		Command::SaveStorage(cmd) => {
			let path = cmd.shared.get_path();

			let snapshot_config = SnapshotConfig::new(path.clone());
			let transport = Transport::Uri(cmd.shared.uri);

			let online_config = OnlineConfig {
				at: cmd.at,
				state_snapshot: Some(snapshot_config),
				pallets: cmd.pallet,
				transport,
				..Default::default()
			};

			let mode = Mode::Online(online_config);

			let builder = Builder::<Block>::new().mode(mode);

			tokio::runtime::Builder::new_current_thread()
				.enable_all()
				.build()
				.unwrap()
				.block_on(async { builder.build().await.unwrap() });

			path
		}
		Command::SaveBlocks(cmd) => {
			let path = cmd.shared.get_path();

			let rpc_service = tokio::runtime::Builder::new_current_thread()
				.enable_all()
				.build()
				.unwrap()
				.block_on(async { scraper::RpcService::new(cmd.shared.uri).await.unwrap() });

			let mut block_arr = Vec::new();

			for block_num in cmd.from..(cmd.from + cmd.num_of_blocks) {
				let block_hash = tokio::runtime::Builder::new_current_thread()
					.enable_all()
					.build()
					.unwrap()
					.block_on(async {
						rpc_service
							.get_block_hash::<hydradx_runtime::Block>(block_num)
							.await
							.unwrap()
					});

				let block = tokio::runtime::Builder::new_current_thread()
					.enable_all()
					.build()
					.unwrap()
					.block_on(async {
						rpc_service
							.get_block::<hydradx_runtime::Block>(block_hash)
							.await
							.unwrap()
					});

				block_arr.push(block);
			}

			scraper::save_blocks_snapshot::<hydradx_runtime::Block>(&block_arr, &path).unwrap();

			path
		}
	};

	println!("The storage file has been saved to {path:?}");
}
