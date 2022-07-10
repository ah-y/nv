use {
   rand::prelude::*,
   std::{os::unix::prelude::CommandExt, process::Command},
};

fn main() {
   let clr = vec!["PaperColor", "edge", "flatui", "iceberg", "nova"];
   let path = std::env::args().nth(1,);
   let color = clr[thread_rng().gen_range(0..clr.len(),)];
   let colo = format!("-c color {color}");
   let _ = match path {
      Some(p,) => Command::new("nvim",).args([&colo, &p,],).exec(),
      None => Command::new("nvim",).args([&colo,],).exec(),
   };
}
