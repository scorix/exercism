#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    compare(first_list, second_list)
}

fn compare<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let mut res = Comparison::Superlist;
    let mut longer_list = first_list;
    let mut shorter_list = second_list;

    if first_list.len() < second_list.len() {
        res = Comparison::Sublist;
        longer_list = second_list;
        shorter_list = first_list;
    }

    if shorter_list.len() == 0 {
        return res;
    }

    for sublist in longer_list.windows(shorter_list.len()) {
        if sublist == shorter_list {
            return res
        }
    }

    Comparison::Unequal
}
