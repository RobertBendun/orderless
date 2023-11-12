extern crate proc_macro;
use proc_macro::*;
use std::iter::FromIterator;

#[proc_macro]
pub fn orderless(items: TokenStream) -> TokenStream {
    let mut items = items.into_iter();
    let Some(TokenTree::Group(group)) = items.next() else {
        todo!("report an error: expected at least one parameter beeing a group")
    };
    let None = items.next() else {
        todo!("report an error: expected at most one paramter")
    };

    let mut ids = vec![];

    let delimiter = group.delimiter();
    let tokens: Vec<_> = group.stream().into_iter().collect();

    for (i, current) in tokens.iter().enumerate() {
        match current {
            TokenTree::Punct(_) => {},
            any => ids.push(i)
        }
    }

    let mut result = vec![];

    let order = ids.clone();

    for (i, permutation) in permute(&mut ids).iter().enumerate()  {
        let mut permuted_tokens = tokens.clone();

        for (old, new) in order.iter().zip(permutation) {
            permuted_tokens[*new] = tokens[*old].clone();
        }

        if i > 0 {
            result.push(TokenTree::Punct(Punct::new('|', Spacing::Alone)));
        }
        result.push(TokenTree::Group(Group::new(
            delimiter,
            TokenStream::from_iter(permuted_tokens)
        )));
    }

    TokenStream::from_iter(result)
}

fn permute<T: Clone>(array: &mut [T]) -> Vec<Vec<T>> {
    let mut permutations = vec![];
    permute_rec(array, 0, &mut permutations);
    permutations
}

fn permute_rec<T: Clone>(array: &mut [T], start: usize, sink: &mut Vec<Vec<T>>) {
    if start == array.len()-1 {
        sink.push(array.to_vec());
        return;
    }

    for i in start .. array.len() {
        array.swap(start, i);
        permute_rec(array, start + 1, sink);
        array.swap(start, i);
    }
}

