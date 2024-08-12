use std::path::Path;
use std::process::Command;

fn main() {
    #[cfg(feature = "webui")]
    {
        let webui_dir = Path::new("webui");
        let webui_src_dir = webui_dir.join("src");

        println!("cargo:rerun-if-changed={}", webui_src_dir.to_str().unwrap());

        // Run "npm install && npm run build" in the webui directory
        for (cmd, args) in [
            ("npm", ["install"].as_slice()),
            ("npm", ["run", "build"].as_slice()),
        ] {
            // Run "npm install" in the webui directory
            let output = Command::new(cmd)
                .args(args)
                .current_dir(webui_dir)
                .output()
                .unwrap_or_else(|_| {
                    panic!(
                        "Failed to execute {} {} in {:?}",
                        cmd,
                        args.join(" "),
                        webui_dir
                    )
                });

            if !output.status.success() {
                panic!(
                    "{} {} failed with output: {}",
                    cmd,
                    args.join(" "),
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            // Optionally print the stdout output if you want to see the build logs
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}
