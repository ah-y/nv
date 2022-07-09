use {
   rand::prelude::*,
   std::{os::unix::prelude::CommandExt, process::Command},
};

fn main() {
   let path = std::env::args().nth(1,);
   let colors = vec!["PaperColor", "flatui", "nova", "iceberg", "edge"];
   let color = colors[thread_rng().gen_range(0..colors.len(),)];
   let colo = format!("-c color {}", color);
   let _ = match path {
      Some(p,) => Command::new("nvim",).args([&colo, &p,],).exec(),
      None => Command::new("nvim",).args([&colo,],).exec(),
   };
}
