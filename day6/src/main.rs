use itertools::Itertools;
use std::fs;

fn detect_marker_start(input: &str, marker_length: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(marker_length)
        .position(|window| window.iter().unique().count() == marker_length)
        .map(|pos| pos + marker_length)
}

fn main() -> color_eyre::Result<()> {
    let input = fs::read_to_string("src/input.txt")?;

    if let Some(result_1) = detect_marker_start(&input, 4) {
        println!("result 1: {:?}", result_1);
    };

    if let Some(result_2) = detect_marker_start(&input, 14) {
        println!("result 2: {:?}", result_2);
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::detect_marker_start;
    use test_case::test_case;

    #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_detect_start_of_packet_marker(index: usize, input: &str) {
        assert_eq!(Some(index), detect_marker_start(input, 4));
    }

    #[test_case(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(23, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(23, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_detect_start_of_message_marker(index: usize, input: &str) {
        assert_eq!(Some(index), detect_marker_start(input, 14));
    }
}
