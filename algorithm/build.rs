// 此处加一个 build.rs 文件， 用于编译 cpp 代码，生成静态库，供 rust 调用。
// vscode 下的   rust-analyzer 会标黄色 ， 但 cargo run 是可以正常编译的。 这是因为 rust-analyzer 没有正确识别 build.rs 文件， 但 cargo 是可以正确处理的。 这不会影响代码的正常运行。

fn main() {
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .flag("-std=c++20")
        .flag("-Wall")
        .flag("-Wextra")
        .include("cpp/include");

    add_cpp_files(&mut build, "cpp"); // 本质也是添加 多个cpp 文件，只是程序化了。
    build.compile("my_ffi"); // 这个是给静态库取一个名字， 生成的静态库会命名为 libmy_ffi.a（在Unix系统上）或 my_ffi.lib（在Windows系统上）。 
    //位置 在 target/debug/build/your_crate_name-xxxxxx/out 目录下。
    // 当add.cpp或build.rs文件发生变化时，重新运行构建脚本
    println!("cargo:rerun-if-changed=src/add.cpp");
}

// Define the add_cpp_files function to add C++ files to the build
fn add_cpp_files(build: &mut cc::Build, dir: &str) {
    use std::fs;
    use std::path::Path;

    let entries = fs::read_dir(dir).expect("Directory not found");
    for entry in entries {
        let path = entry.expect("Invalid entry").path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("cpp") {
            build.file(path);
        }
    }
}
