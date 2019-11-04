use regex::CaptureMatches;

pub fn join_args(iter: CaptureMatches) -> String {
    iter
        .map(|x| x.get(0).unwrap().as_str())
        .collect::<String>()
}

#[macro_export]
macro_rules! command_hashmap {
    ($( $command:expr ), *) => {{
        let mut hashmap = HashMap::new();
        $( hashmap.extend($command.get_info().names.into_iter().map(|name| (name, &$command as &dyn Command))); )*
        hashmap
    }};
}