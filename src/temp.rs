use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use serde_json::{self, Value};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: pkgmngr install <package_name>");
        return Ok(());
    }
    let command = &args[1];
    let package_name = if args.len() > 2 {
        &args[2]
    } else {
        println!("Usage: pkgmngr install <package_name>");
        return Ok(());
    };

    let packages_path = Path::new("./packages.json");
    let installed_path = Path::new("./installed.json");

    let packages_data = fs::read_to_string(packages_path)?;
    let packages: Value = serde_json::from_str(&packages_data)?;

    let installed_data = fs::read_to_string(installed_path)?;
    let mut installed: Vec<String> = serde_json::from_str(&installed_data)?;

    if command != "install" {
        println!("Error: unknown command '{}'", command);
        return Ok(());
    }

    if packages.get(package_name).is_none() {
        println!("Error: package '{}' not found", package_name);
    } else {
        if !installed.contains(package_name) {
            let package_name_str = packages[package_name][0].to_string();
            let package_name_str = package_name_str.trim_matches('"').to_string();
            let url = format!("https://raw.githubusercontent.com/Cipher-Linux/cr/main/packages/{}", package_name_str);
            let mut child = Command::new("wget")
                .arg(url)
                .arg("--directory-prefix=./installed")
                .spawn()?;
            let _output = child.wait()?;
            let package_bash_str = packages[package_name][2].to_string();
            let package_bash_str = package_bash_str.trim_matches('"').to_string();
            let pkg_bash_sh = packages[package_name][3].to_string();
            let pkg_bash_sh = pkg_bash_sh.trim_matches('"').to_string();
            let shurl = format!("https://raw.githubusercontent.com/Cipher-Linux/cr/main/packages/{}", package_bash_str);
            let mut child2 = Command::new("wget")
                .arg(shurl)
                .arg("--directory-prefix=./installed")
                .spawn()?;
            let _output2 = child2.wait()?;
            let mut child3 = Command::new("bash")
                .arg(pkg_bash_sh)
                .spawn()?;
            let _output3 = child3.wait()?;
            installed.push(package_name.to_string());
            let installed_data = serde_json::to_string(&installed)?;
            fs::write(installed_path, installed_data)?;
        } else {
            println!("package already exists");
        }
    }

    Ok(())
}
