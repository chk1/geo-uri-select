#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn test_bing_re() {
        let test_strings = [
            "bingmaps:?cp=47.6204~-122.3491",
            "bingmaps:?cp=47.6204~-122.3491&lvl=10",
            "bingmaps:?collection=point.47.6204_-122.3491_Caesars%20Palace&lvl=16",
            "bingmaps:?collection=point.47.6204_-122.3491_Some%255FBusiness",
            "bingmaps:?cp=47.6204~-122.3491&trfc=1&sty=a",
            "bingmaps:?cp=47.6204~-122.3491&sty=3d",
            "bingmaps:?cp=47.6204~-122.3491&sty=3d&rad=200&pit=75&hdg=165",
            "bingmaps:?cp=47.6204~-122.3491&ss=1"
        ];
        let re_bing = Regex::new(r"^bingmaps:(.+)(collection=point\.|cp=)(?P<lat>-?[0-9]+(\.[0-9]+)?)[_~](?P<lon>-?[0-9]+(\.[0-9]+)?)").unwrap();
        for test_string in test_strings {
            assert_eq!(re_bing.is_match(test_string), true);
            let cap = re_bing.captures(test_string).unwrap();
            assert_eq!(cap.name("lat").unwrap().as_str().eq("47.6204"), true);
            assert_eq!(cap.name("lon").unwrap().as_str().eq("-122.3491"), true);
        }
    }
}
