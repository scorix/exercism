#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len() {
        0 => Comparison::Sublist,
        x if x < second_list.len() => expect(first_list, second_list, Comparison::Sublist),
        x if x == second_list.len() => expect(first_list, second_list, Comparison::Equal),
        x if x > second_list.len() => expect(first_list, second_list, Comparison::Superlist),
        _ => Comparison::Unequal,
    }
}

fn expect<T: PartialEq>(first_list: &[T], second_list: &[T], comparison: Comparison) -> Comparison {
    let expected = match comparison {
        Comparison::Equal => first_list == second_list,
        Comparison::Sublist => contains(first_list, second_list),
        Comparison::Superlist => contains(second_list, first_list),
        _ => true,
    };

    if expected {
        return comparison;
    }

    Comparison::Unequal
}

fn contains<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    match first_list.len() {
        0 => return true,
        x if x >= second_list.len() => return false,
        _ => (),
    }

    second_list
        .windows(first_list.len())
        .any(|window| window == first_list)
}
