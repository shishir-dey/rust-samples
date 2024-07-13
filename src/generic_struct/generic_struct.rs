fn main() {
    struct Pair<K, V> {
        key: K,
        value: V,
    }

    let data_1: Pair<u8, String> = Pair {
        key: 2,
        value: "Abc".to_string(),
    };
    println!("Data_1 Key: {} Value: {}", data_1.key, data_1.value);
}
