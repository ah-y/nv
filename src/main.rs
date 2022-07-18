use {
   rand::prelude::*,
   std::{os::unix::prelude::CommandExt, process::Command},
};

fn main() {
   let clr = vec!["PaperColor", "edge", "flatui", "iceberg", "nova", "kalisi", "nord"];
   let path = std::env::args().nth(1,).unwrap_or("/Users/r/.config/nvim/init.vim".to_string(),);
   let color = clr[thread_rng().gen_range(0..clr.len(),)];
   let colo = format!("-c color {color}");
   Command::new("nvim",).args([&colo, &path,],).exec();
}
