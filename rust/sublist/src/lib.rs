#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.len() == second_list.len() {
        return compare_equal(first_list, second_list);
    }

    if first_list.len() > second_list.len() {
        if compare_sublist(second_list, first_list) == Comparison::Sublist {
            return Comparison::Superlist;
        }
    }

    compare_sublist(first_list, second_list)
}

fn compare_equal<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.len() == 0 {
        return Comparison::Equal;
    }

    for idx in 0..first_list.len() {
        if first_list[idx] != second_list[idx] {
            return Comparison::Unequal;
        }
    }

    Comparison::Equal
}

fn compare_sublist<T: PartialEq>(shorter_list: &[T], longer_list: &[T]) -> Comparison {
    if shorter_list.len() == 0 {
        return Comparison::Sublist;
    }

    for list in longer_list.windows(shorter_list.len()) {
        match compare_equal(list, shorter_list) {
            Comparison::Equal => return Comparison::Sublist,
            _ => continue
        }
    }

    Comparison::Unequal
}
