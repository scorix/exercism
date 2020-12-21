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

    let mut result = Comparison::Sublist;

    for idx in 0..longer_list.len() {
        for shorter_idx in 0..shorter_list.len() {
            if idx + shorter_idx >= longer_list.len() {
                break;
            }
            if longer_list[idx + shorter_idx] != shorter_list[shorter_idx] {
                result = Comparison::Unequal;
                break;
            }
            if shorter_idx == shorter_list.len() - 1 {
                result = Comparison::Sublist;
            }
        }

        match result {
            Comparison::Sublist => return Comparison::Sublist,
            _ => continue,
        }
    }

    Comparison::Unequal
}
