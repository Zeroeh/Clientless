#[cfg(test)]
mod tests {
    use realmlib::network;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_net() {
        network::netstat();
    }

    #[test]
    fn test_stat_fn() {
        //test stats and make sure our functions on them work as intended
        let x = network::types::StatData::new();
        assert!(x.is_string_stat() == true);
        let y = network::types::StatData::new();
        assert!(y.is_string_stat() == false);
    }
}
