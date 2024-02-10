fn main() {
    lalrpop::Configuration::new()
        .process_file("src/parser/whilep.lalrpop")
        .unwrap();
}
