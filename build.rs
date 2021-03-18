fn main() {     
        std::process::Command::new("packfolder.exe").args(&["src/frontend", "dupa.rc", "-binary"])
    .output().expect("no i ciul");
}